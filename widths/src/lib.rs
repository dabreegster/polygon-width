mod join_lines;
mod mercator;
mod step_along_line;
pub mod utils;

use geo::{
    Area, Contains, Coord, EuclideanDistance, EuclideanLength, Line, LineInterpolatePoint,
    LineIntersection, LineLocatePoint, LineString, Polygon,
};
pub use mercator::Mercator;

pub struct Pavement {
    // input
    pub polygon: Polygon,

    // should be center line
    pub skeletons: Vec<LineString>,

    // regularly spaced lines that measure width
    pub perp_lines: Vec<Line>,

    // thickened center lines, along with their width
    pub thickened_lines: Vec<(Polygon, f64)>,
}

impl Pavement {
    pub fn new(mut polygon: Polygon) -> Self {
        // Remove small holes, representing bus stops in the example input
        if true {
            let (exterior, mut holes) = polygon.into_inner();
            holes.retain(|hole| {
                let p = Polygon::new(hole.clone(), Vec::new());
                p.unsigned_area() > 100.0
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

    pub fn calculate(&mut self) {
        self.skeletonize();
        self.make_perp_lines();
    }

    fn skeletonize(&mut self) {
        let avoid_boundaries_threshold = 0.1;

        let mut skeletons = Vec::new();
        // TODO true/false here seems to depend on using Mercator
        for line in geo_buffer::skeleton_of_polygon_to_linestring(&self.polygon, false) {
            // There are some huge lines that totally escape the polygon.
            if !self.polygon.contains(&line) {
                continue;
            }

            // There are also perpendicular straight skeleton segments that don't represent the
            // center-line. Measure the distance between each line endpoint and the polygon's
            // boundaries. If any is too small, skip it.
            let mut ok = true;
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
            if ok {
                skeletons.push(line);
            }
        }

        self.skeletons = crate::join_lines::join_linestrings(skeletons);
    }

    fn make_perp_lines(&mut self) {
        let step_size_meters = 5.0;
        let project_away_meters = 10.0;

        for skeleton in &self.skeletons {
            for (pt, angle) in crate::step_along_line::step_along_line(skeleton, step_size_meters) {
                let pt1 = project_away(pt, angle - 90.0, project_away_meters);
                let pt2 = project_away(pt, angle + 90.0, project_away_meters);

                let Some(perp) = clip_line_to_polygon(&self.polygon, Line::new(pt1, pt2)) else {
                    continue;
                };
                let width = perp.euclidean_length();
                // TODO Oh hey, happens to be a good heuristic to prune out weird stuff?!
                if width == 0.0 {
                    continue;
                }

                self.perp_lines.push(perp);

                self.thickened_lines.push((
                    thicken(
                        Line::new(pt, project_away(pt, angle, step_size_meters)),
                        width,
                    ),
                    width,
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

fn thicken(line: Line, width: f64) -> Polygon {
    let angle = line_angle_degrees(line);
    Polygon::new(
        LineString::new(vec![
            project_away(line.start, angle - 90.0, width / 2.0),
            project_away(line.start, angle + 90.0, width / 2.0),
            project_away(line.end, angle + 90.0, width / 2.0),
            project_away(line.end, angle - 90.0, width / 2.0),
            project_away(line.start, angle - 90.0, width / 2.0),
        ]),
        Vec::new(),
    )
}

fn line_angle_degrees(line: Line) -> f64 {
    line.dy().atan2(line.dx()).to_degrees()
}
