use geo::{Coord, LineLocatePoint, LineSplit, LineString};

pub fn split(
    linestring: &LineString,
    thickened_points: Vec<(Coord, f64)>,
    width_granularity: f64,
) -> Vec<(LineString, f64)> {
    let mut result = Vec::new();

    // TODO itertools coalesce or chunks_by are both temptingly close to being useful, but...
    let (mut start_pt, mut start_width) = thickened_points[0];
    let (mut end_pt, _) = thickened_points[0];
    let len = thickened_points.len();
    for (idx, (pt, width)) in thickened_points.into_iter().enumerate() {
        // Do this first because of the last point case
        if (start_width - width).abs() <= width_granularity {
            end_pt = pt;
        }

        if idx == len - 1 || (start_width - width).abs() > width_granularity {
            if let Some(sliced) = slice(linestring, start_pt, end_pt) {
                // Could try to average the width in this range
                result.push((sliced, start_width));
                start_pt = pt;
                start_width = width;
            }
        }
    }

    result
}

fn slice(linestring: &LineString, pt1: Coord, pt2: Coord) -> Option<LineString> {
    if pt1 == pt2 {
        return None;
    }
    let frac1 = linestring.line_locate_point(&pt1.into())?;
    let frac2 = linestring.line_locate_point(&pt2.into())?;
    let result = linestring.line_split_twice(frac1, frac2)?;
    result.into_second()
}
