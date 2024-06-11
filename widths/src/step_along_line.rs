use geo::{Coord, EuclideanLength, Line, LineInterpolatePoint, LineString};

// TODO Upstream to geo

/// Walks along a linestring at regular intervals and output the point and angle of the line in
/// degrees. This can't use
/// https://docs.rs/geo/latest/geo/algorithm/line_interpolate_point/trait.LineInterpolatePoint.html
/// because the line / angle isn't returned.
pub fn step_along_line(linestring: &LineString, interval: f64) -> Vec<(Coord, f64)> {
    // TODO This is very inefficient; it keeps searching from the start of the whole linestring
    let mut result = Vec::new();
    let mut dist_along = 0.0;
    let length = linestring.euclidean_length();
    while dist_along < length {
        result.push(dist_along_linestring(linestring, dist_along));
        dist_along += interval;
    }
    result
}

fn dist_along_linestring(linestring: &LineString, dist: f64) -> (Coord, f64) {
    let mut dist_left = dist;
    for line in linestring.lines() {
        let length = line.euclidean_length();
        if length == 0.0 {
            continue;
        }
        if dist_left <= length {
            return (dist_along_line(line, dist_left), line_angle_degrees(line));
        }
        dist_left -= length;
    }
    // If there's leftover, it's just a small epsilon
    let line = linestring.lines().last().unwrap();
    (line.end, line_angle_degrees(line))
}

fn dist_along_line(line: Line, dist: f64) -> Coord {
    line.line_interpolate_point(dist / line.euclidean_length())
        .unwrap()
        .into()
}

fn line_angle_degrees(line: Line) -> f64 {
    line.dy().atan2(line.dx()).to_degrees()
}
