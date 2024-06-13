mod join_lines;
mod mercator;
mod step_along_line;
pub mod utils;

use geo::{
    Area, Contains, Coord, EuclideanDistance, EuclideanLength, Line, LineInterpolatePoint,
    LineIntersection, LineLocatePoint, LineString, Polygon,
};
pub use mercator::Mercator;
use serde::Deserialize;

pub struct Pavement {
    // input
    pub polygon: Polygon,

    // should be center line
    pub skeletons: Vec<LineString>,

    // regularly spaced lines that measure width
    pub perp_lines: Vec<Line>,

    // thickened center lines, along with their width at each end
    pub thickened_lines: Vec<(Polygon, f64, f64)>,
}

impl Pavement {
    pub fn new(mut polygon: Polygon, cfg: &Config) -> Self {
        // Remove small holes, representing bus stops in the example input
        if let Some(limit) = cfg.remove_holes {
            let (exterior, mut holes) = polygon.into_inner();
            holes.retain(|hole| {
                let p = Polygon::new(hole.clone(), Vec::new());
                p.unsigned_area() > limit
            });
            polygon = Polygon::new(exterior, holes);
        }

        Self {
            polygon,
            skeletons: Vec::new(),
            perp_lines: Vec::new(),
            thickened_lines: Vec::new(),
        }
    }

    pub fn calculate(&mut self, cfg: &Config) {
        self.skeletonize(cfg);
        if let Some(step_size) = cfg.make_perps_step_size {
            self.make_perp_lines(step_size);
        }
    }

    fn skeletonize(&mut self, cfg: &Config) {
        let mut skeletons = Vec::new();

        // TODO We want the lines inside, but this seems to give the wrong answer for some inputs,
        // even after reorienting. Just try both.
        for orientation in [true, false] {
            for line in geo_buffer::skeleton_of_polygon_to_linestring(&self.polygon, orientation) {
                // There are some huge lines that totally escape the polygon.
                if cfg.filter_skeletons_outside && !self.polygon.contains(&line) {
                    continue;
                }

                // There are also perpendicular straight skeleton segments that don't represent the
                // center-line. Measure the distance between each line endpoint and the polygon's
                // boundaries. If any is too small, skip it.
                let mut ok = true;
                if let Some(avoid_boundaries_threshold) = cfg.filter_skeletons_near_boundary {
                    for pt1 in [line.points().next().unwrap(), line.points().last().unwrap()] {
                        for boundary in vec![self.polygon.exterior()]
                            .into_iter()
                            .chain(self.polygon.interiors())
                        {
                            // TODO Could try ClosestPoint again
                            let fraction = boundary.line_locate_point(&pt1).unwrap();
                            let pt2 = boundary.line_interpolate_point(fraction).unwrap();

                            if pt1.euclidean_distance(&pt2) < avoid_boundaries_threshold {
                                ok = false;
                            }
                        }
                    }
                }
                if ok {
                    skeletons.push(line);
                }
            }

            // Try the next orientation
            if skeletons.is_empty() {
                continue;
            }

            if cfg.join_skeletons {
                self.skeletons = crate::join_lines::join_linestrings(skeletons);
            } else {
                self.skeletons = skeletons;
            }

            if let Some(threshold) = cfg.remove_short_skeletons {
                let longest_len = self
                    .skeletons
                    .iter()
                    .map(|ls| ls.euclidean_length())
                    .max_by_key(|len| (len * 1000.0) as usize)
                    .unwrap();
                self.skeletons
                    .retain(|ls| ls.euclidean_length() / longest_len >= threshold);
            }

            break;
        }
    }

    fn make_perp_lines(&mut self, step_size_meters: f64) {
        let project_away_meters = 10.0;

        for skeleton in &self.skeletons {
            let mut thickened_points = Vec::new();
            for (pt, angle) in crate::step_along_line::step_along_line(skeleton, step_size_meters) {
                let pt1 = project_away(pt, angle - 90.0, project_away_meters);
                let pt2 = project_away(pt, angle + 90.0, project_away_meters);

                let Some(perp) = clip_line_to_polygon(&self.polygon, Line::new(pt1, pt2)) else {
                    continue;
                };
                let width = perp.euclidean_length();
                // TODO Oh hey, happens to be a good heuristic to prune out weird stuff?!
                if width == 0.0 {
                    // TODO remove this skeleton???
                    continue;
                }

                self.perp_lines.push(perp);

                thickened_points.push((pt, angle, width));
            }

            // Make thickened polygons that may have different widths on each end
            // TODO Make sure we have points at the very start and end. Ideally we do that with
            // step_along_line
            for pair in thickened_points.windows(2) {
                let (pt1, angle1, width1) = pair[0];
                let (pt2, angle2, width2) = pair[1];

                self.thickened_lines.push((
                    Polygon::new(
                        LineString::new(vec![
                            project_away(pt1, angle1 - 90.0, width1 / 2.0),
                            project_away(pt1, angle1 + 90.0, width1 / 2.0),
                            project_away(pt2, angle2 + 90.0, width2 / 2.0),
                            project_away(pt2, angle2 - 90.0, width2 / 2.0),
                            project_away(pt1, angle1 - 90.0, width1 / 2.0),
                        ]),
                        Vec::new(),
                    ),
                    width1,
                    width2,
                ));
            }
        }
    }
}

fn project_away(pt: Coord, angle_degrees: f64, distance: f64) -> Coord {
    let (sin, cos) = angle_degrees.to_radians().sin_cos();
    Coord {
        x: pt.x + distance * cos,
        y: pt.y + distance * sin,
    }
}

fn clip_line_to_polygon(polygon: &Polygon, line: Line) -> Option<Line> {
    let mut hits = Vec::new();
    for polygon_line in polygon.exterior().lines() {
        if let Some(LineIntersection::SinglePoint { intersection, .. }) =
            geo::algorithm::line_intersection::line_intersection(line, polygon_line)
        {
            hits.push(intersection);
        }
    }
    if hits.len() == 2 {
        return Some(Line::new(hits[0], hits[1]));
    }
    None
}

#[derive(Deserialize)]
pub struct Config {
    // Remove smaller than this unsigned area in m^2
    pub remove_holes: Option<f64>,

    pub filter_skeletons_outside: bool,
    pub filter_skeletons_near_boundary: Option<f64>,
    pub join_skeletons: bool,
    // When the ratio of a line to the longest line is less than this threshold, remove it
    pub remove_short_skeletons: Option<f64>,

    pub make_perps_step_size: Option<f64>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            remove_holes: Some(100.0),

            filter_skeletons_outside: true,
            filter_skeletons_near_boundary: Some(0.1),
            join_skeletons: true,
            remove_short_skeletons: Some(0.1),

            make_perps_step_size: Some(5.0),
        }
    }
}
