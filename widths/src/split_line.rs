use geo::{Coord, LineLocatePoint, LineSplit, LineString};

/// Splits the line every time the width changes by some granularity. Returns the min and max width
/// of each split.
pub fn split(
    linestring: &LineString,
    thickened_points: Vec<(Coord, f64)>,
    width_granularity: f64,
) -> Vec<(LineString, f64, f64)> {
    let mut result = Vec::new();

    let mut idx1 = 0;
    while idx1 < thickened_points.len() - 1 {
        let mut idx2 = idx1 + 1;
        while (thickened_points[idx1].1 - thickened_points[idx2].1).abs() <= width_granularity
            && idx2 < thickened_points.len() - 1
        {
            idx2 += 1;
        }
        if let Some(sliced) = slice(
            linestring,
            thickened_points[idx1].0,
            thickened_points[idx2].0,
        ) {
            let min = thickened_points[idx1..=idx2]
                .iter()
                .map(|pair| pair.1)
                .min_by_key(|w| (w * 1000.0) as usize)
                .unwrap();
            let max = thickened_points[idx1..=idx2]
                .iter()
                .map(|pair| pair.1)
                .max_by_key(|w| (w * 1000.0) as usize)
                .unwrap();
            result.push((sliced, min, max));
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
