use std::collections::BTreeSet;

use geo::{Coord, EuclideanLength, LineString};
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HashedPoint(isize, isize);

#[derive(Clone, Copy, PartialEq)]
struct EdgeIdx(usize);

impl HashedPoint {
    fn new(pt: Coord) -> Self {
        Self((pt.x * 1_000_000.0) as isize, (pt.y * 1_000_000.0) as isize)
    }
}

pub fn join_linestrings(mut lines: Vec<LineString>) -> Vec<LineString> {
    loop {
        // Build a graph from the lines
        let mut intersections: BTreeSet<HashedPoint> = BTreeSet::new();
        let mut graph: UnGraphMap<HashedPoint, EdgeIdx> = UnGraphMap::new();

        for (idx, line) in lines.iter().enumerate() {
            let i1 = HashedPoint::new(*line.0.first().unwrap());
            let i2 = HashedPoint::new(*line.0.last().unwrap());
            intersections.insert(i1);
            intersections.insert(i2);
            graph.add_edge(i1, i2, EdgeIdx(idx));
        }

        if let Some(path) = find_longest_path(&graph, &lines, &intersections) {
            lines = join_path(lines, path);
        } else {
            return lines;
        }
    }
}

// Of length > 1
fn find_longest_path(
    graph: &UnGraphMap<HashedPoint, EdgeIdx>,
    edges: &Vec<LineString>,
    intersections: &BTreeSet<HashedPoint>,
) -> Option<Vec<EdgeIdx>> {
    let mut best_path = Vec::new();
    let mut best_length = 0.0;

    // If we had DAGs, we could try Dijkstra with negative edge weights. For now, just brute-force
    // it -- the graphs should be tiny
    for src in intersections {
        for dst in intersections {
            if src == dst {
                continue;
            }
            if let Some((length, path)) = petgraph::algo::astar(
                graph,
                *src,
                |i| i == *dst,
                |(_, _, idx)| edges[idx.0].euclidean_length(),
                |_| 0.0,
            ) {
                if path.len() > 2 && length > best_length {
                    best_length = length;
                    best_path = path;
                }
            }
        }
    }

    let mut result = Vec::new();
    for pair in best_path.windows(2) {
        result.push(*graph.edge_weight(pair[0], pair[1]).unwrap());
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

// Combines everything in the path, returning a smaller list of lines
fn join_path(lines: Vec<LineString>, path: Vec<EdgeIdx>) -> Vec<LineString> {
    // Build up the joined line
    let mut points = Vec::new();
    for idx in &path {
        let mut next = lines[idx.0].clone().into_inner();
        if points.is_empty() {
            points = next;
            continue;
        }
        let pt1 = HashedPoint::new(*points.first().unwrap());
        let pt2 = HashedPoint::new(*points.last().unwrap());
        let pt3 = HashedPoint::new(*next.first().unwrap());
        let pt4 = HashedPoint::new(*next.last().unwrap());

        if pt1 == pt3 {
            points.reverse();
            points.pop();
            points.extend(next);
        } else if pt1 == pt4 {
            next.pop();
            next.extend(points);
            points = next;
        } else if pt2 == pt3 {
            points.pop();
            points.extend(next);
        } else if pt2 == pt4 {
            next.reverse();
            points.pop();
            points.extend(next);
        } else {
            unreachable!()
        }
    }
    let joined = LineString::new(points);
    let mut result = vec![joined];
    for (i, line) in lines.into_iter().enumerate() {
        if !path.contains(&EdgeIdx(i)) {
            result.push(line);
        }
    }
    result
}
