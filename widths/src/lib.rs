mod join_lines;
mod mercator;
mod split_line;
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

    // The center line and its width, split up into when the width changes past some threshold
    pub center_with_width: Vec<(LineString, f64)>,
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
            center_with_width: Vec::new(),
        }
    }

    pub fn calculate(&mut self, cfg: &Config) {
        self.skeletonize(cfg);
        self.make_perp_lines(cfg);
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

    fn make_perp_lines(&mut self, cfg: &Config) {
        let Some(step_size_meters) = cfg.make_perps_step_size else {
            return;
        };
        let project_away_meters = 100.0;

        for skeleton in &self.skeletons {
            let mut thickened_points = Vec::new();
            for (pt, angle) in crate::step_along_line::step_along_line(skeleton, step_size_meters) {
                let pt1 = project_away(pt, angle - 90.0, project_away_meters);
                let pt2 = project_away(pt, angle + 90.0, project_away_meters);

                let Some(perp) = clip_line_to_polygon(&self.polygon, pt, Line::new(pt1, pt2), cfg)
                else {
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
            if thickened_points.len() < 2 {
                continue;
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

            self.center_with_width.extend(split_line::split(
                skeleton,
                thickened_points
                    .into_iter()
                    .map(|(pt, _, width)| (pt, width))
                    .collect(),
                cfg.width_granularity,
            ));
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

fn clip_line_to_polygon(
    polygon: &Polygon,
    midpoint: Coord,
    line: Line,
    cfg: &Config,
) -> Option<Line> {
    let mut hits = Vec::new();
    for boundary in vec![polygon.exterior()]
        .into_iter()
        .chain(polygon.interiors())
    {
        for polygon_line in boundary.lines() {
            if let Some(LineIntersection::SinglePoint { intersection, .. }) =
                geo::algorithm::line_intersection::line_intersection(line, polygon_line)
            {
                hits.push(intersection);
            }
        }
    }

    // The line might hit the polygon at more than 2 points. Find the two closest hits to the
    // the midpoint (of the clipped line we should return)
    hits.sort_by_key(|pt| (midpoint.euclidean_distance(pt) * 1000.0) as usize);
    if hits.len() < 2 {
        return None;
    }

    // Check if midpoint is really acting like a midpoint. There are false positives near sharp
    // corners, where the left and right projection are very different. This requires midpoint to
    // really be on the polygon's center line.
    if let Some(threshold) = cfg.perp_midpoint_ratio {
        let mut dist1 = hits[0].euclidean_distance(&midpoint);
        let mut dist2 = hits[1].euclidean_distance(&midpoint);
        if dist1 > dist2 {
            std::mem::swap(&mut dist1, &mut dist2);
        }
        if dist1 / dist2 < threshold {
            return None;
        }
    }

    Some(Line::new(hits[0], hits[1]))
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
    pub perp_midpoint_ratio: Option<f64>,

    // For producing center_with_width, split the line when width differs by more than this amount
    pub width_granularity: f64,
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
            perp_midpoint_ratio: Some(0.5),

            width_granularity: 0.5,
        }
    }
}
