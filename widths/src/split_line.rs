use geo::{Coord, LineLocatePoint, LineSplit, LineString};

pub fn split(
    linestring: &LineString,
    thickened_points: Vec<(Coord, f64)>,
    width_granularity: f64,
) -> Vec<(LineString, f64)> {
    let mut result = Vec::new();

    let mut idx1 = 0;
    while idx1 < thickened_points.len() - 1 {
        let mut idx2 = idx1 + 1;
        while (thickened_points[idx1].1 - thickened_points[idx2].1).abs() <= width_granularity {
            idx2 += 1;
        }
        if let Some(sliced) = slice(
            linestring,
            thickened_points[idx1].0,
            thickened_points[idx2].0,
        ) {
            // TODO Output the range of widths, or just the one, or the average?
            result.push((sliced, thickened_points[idx1].1));
        }
        idx1 = idx2;
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

// TODO Unit tests
