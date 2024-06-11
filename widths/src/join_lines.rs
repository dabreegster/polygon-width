use geo::LineString;

// TODO Upstream to geo

// TODO This is chatgpt output that seems sane to me, but I think it needs to be a fixed-point
// algorithm and keep joining until there are no changes. Maybe union-find would help. Also need to
// be careful near loops (or remove holes first).
pub fn join_linestrings(lines: Vec<LineString>) -> Vec<LineString> {
    let mut result = Vec::new();
    let mut used = vec![false; lines.len()];

    for i in 0..lines.len() {
        if used[i] {
            continue;
        }

        let mut current = lines[i].clone();
        used[i] = true;

        loop {
            let mut merged = false;

            for j in 0..lines.len() {
                if used[j] {
                    continue;
                }

                if has_common_endpoint(&current, &lines[j]) {
                    current = join(current, lines[j].clone());
                    used[j] = true;
                    merged = true;
                    break;
                }
            }

            if !merged {
                break;
            }
        }

        result.push(current);
    }

    result
}

fn has_common_endpoint(ls1: &LineString, ls2: &LineString) -> bool {
    let start1 = ls1.0[0];
    let end1 = ls1.0[ls1.0.len() - 1];
    let start2 = ls2.0[0];
    let end2 = ls2.0[ls2.0.len() - 1];

    start1 == start2 || start1 == end2 || end1 == start2 || end1 == end2
}

// Must be called when share_endpoint is true. The order of points might change arbitrarily.
fn join(ls1: LineString, ls2: LineString) -> LineString {
    let mut coords1 = ls1.into_inner();
    let mut coords2 = ls2.into_inner();

    // TODO Fuzzy comparison?
    if coords1.first() == coords2.first() {
        coords1.reverse();
        coords1.pop();
        coords1.extend(coords2);
        LineString::from(coords1)
    } else if coords1.first() == coords2.last() {
        coords2.pop();
        coords2.extend(coords1);
        LineString::from(coords2)
    } else if coords1.last() == coords2.first() {
        coords1.pop();
        coords1.extend(coords2);
        LineString::from(coords1)
    } else if coords1.last() == coords2.last() {
        coords2.reverse();
        coords1.pop();
        coords1.extend(coords2);
        LineString::from(coords1)
    } else {
        unreachable!()
    }
}
