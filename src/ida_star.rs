use super::heuristics::Heuristic;
use super::puzzle::Puzzle;
use super::solution::Solution;

pub enum SearchResult {
    Found,
    NotFound,
    Minimum(u32),
}

// [https://en.wikipedia.org/wiki/Iterative_deepening_A*]
// [https://github.com/samueltardieu/pathfinding/blob/main/src/directed/idastar.rs]
//
pub fn ida_star(
    start: Puzzle,
    end: Puzzle,
    h: &dyn Heuristic,
) -> Option<Solution> {
    let mut bound = h.first_time(&start, &end);
    let mut path = vec![start.clone()];

    loop {
        match search(&mut path, 0, bound, &end, h) {
            SearchResult::Found => {
                return Some(Solution {
                    total_opened: 0,
                    max_states: 0,
                    path,
                })
            }
            SearchResult::Minimum(m) => bound = m,
            SearchResult::NotFound => return None,
        }
    }
}

fn search(
    path: &mut Vec<Puzzle>,
    g: u32,
    bound: u32,
    end: &Puzzle,
    h: &dyn Heuristic,
) -> SearchResult {
    let start = path.last().unwrap();
    let f = g + h.first_time(&start, &end);
    if f > bound {
        return SearchResult::Minimum(f);
    }
    if start == end {
        return SearchResult::Found;
    }
    let mut min = None;
    let mut neighbors = start
        .neighbors()
        .into_iter()
        .filter_map(|p| {
            if path.contains(&p) {
                None
            } else {
                Some((p.clone(), g + h.first_time(&p, end) + 1))
            }
        })
        .collect::<Vec<_>>();
    neighbors.sort_by_key(|&(_, c)| c);
    for (node, _) in neighbors {
        path.push(node);
        let t = search(path, g + 1, bound, &end, h);
        match t {
            SearchResult::Minimum(m) => match min {
                None => min = Some(m),
                Some(n) if m < n => min = Some(m),
                Some(_) => (),
            },
            SearchResult::Found => return SearchResult::Found,
            SearchResult::NotFound => (),
        };
        path.pop();
    }

    match min {
        Some(m) => SearchResult::Minimum(m),
        None => SearchResult::NotFound,
    }
}
