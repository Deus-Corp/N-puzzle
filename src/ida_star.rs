use super::puzzle::Puzzle;
use super::solution::Solution;

pub enum SearchResult {
    Found,
    NotFound,
    Minimum(u32),
}

// [https://en.wikipedia.org/wiki/Iterative_deepening_A*]
//
pub fn ida_star(
    start: Puzzle,
    end: Puzzle,
    h: Box<dyn Fn(&Puzzle, &Puzzle) -> u32>,
) -> Option<Solution> {
    let mut bound = h(&start, &end);
    let mut path = vec![start.clone()];

    loop {
        match search(&mut path, 0, bound, &start, &end, &h) {
            SearchResult::Minimum(m) => bound = m,
            SearchResult::Found => {
                return Some(Solution {
                    total_opened: 0,
                    max_states: 0,
                    path,
                })
            }
            SearchResult::NotFound => return None,
        }
    }
}

fn search(
    path: &mut Vec<Puzzle>,
    g: u32,
    bound: u32,
    start: &Puzzle,
    end: &Puzzle,
    h: &Box<dyn Fn(&Puzzle, &Puzzle) -> u32>,
) -> SearchResult {
    let mut min = SearchResult::NotFound;
    let node = path.last().unwrap();
    let f = g + h(&start, &end);
    if f > bound {
        return SearchResult::Minimum(f);
    }
    if node == end {
        return SearchResult::Found;
    }
    for neighbor in node.neighbors() {
        if !path.contains(&neighbor) {
            path.push(neighbor);
            let t = search(path, 0, bound, &start, &end, h);
            match t {
                SearchResult::Minimum(m) => match min {
                    SearchResult::Minimum(min_value) => {
                        if m < min_value {
                            min = SearchResult::Minimum(m)
                        }
                    }
                    _ => {
                        min = SearchResult::Minimum(m);
                    }
                },
                s => return s,
            };
            path.pop();
        }
    }

    min
}
