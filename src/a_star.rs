use super::puzzle::Puzzle;
use std::collections::{BinaryHeap, HashMap};

const TRANSITION_COST: u32 = 1;

pub fn reconstruct_path(
    came_from: HashMap<Puzzle, Puzzle>,
    current: Puzzle,
) -> Vec<Puzzle> {
    let mut path = vec![];
    path.push(current.clone());

    let mut iter = &current;
    while came_from.contains_key(iter) {
        iter = &came_from[iter];
        path.push(iter.clone());
    }
    path.reverse();
    path
}

// [https://github.com/samueltardieu/pathfinding/blob/main/src/directed/astar.rs]
//
pub fn a_star(
    start: Puzzle,
    goal: Puzzle,
    h: Box<dyn Fn(&Puzzle, &Puzzle) -> u32>,
) -> Option<Vec<Puzzle>> {
    let mut open_list = BinaryHeap::new();
    let mut closed_set = vec![];
    let mut came_from = HashMap::new();
    open_list.push(Score {
        puzzle: start.clone(),
        g: 0,
        f: h(&start, &goal),
    });

    while let Some(current) = open_list.pop() {
        if current.puzzle == goal {
            let path = reconstruct_path(came_from, current.puzzle);
            return Some(path);
        }
        closed_set.push(current.puzzle.clone());
        for neighbor in current.puzzle.neighbors() {
            if closed_set.contains(&neighbor) {
                continue;
            }
            came_from.insert(neighbor.clone(), current.puzzle.clone());
            let g = current.g + TRANSITION_COST;
            let f = g + h(&neighbor, &goal);
            open_list.push(Score {
                puzzle: neighbor,
                f,
                g,
            });
        }
    }
    None
}

#[derive(PartialEq, Eq)]
struct Score {
    puzzle: Puzzle,
    g: u32,
    f: u32,
}

use std::cmp::Ordering;

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
