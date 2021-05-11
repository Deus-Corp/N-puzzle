use super::a_star::a_star;
use super::puzzle::Puzzle;

pub struct Solution {
    pub total_opened: usize,
    pub max_states: usize,
    pub path: Vec<Puzzle>,
}

use std::fmt;

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Solution:")?;
        for puzzle in &self.path {
            for chunk in puzzle.flat.chunks(puzzle.n) {
                writeln!(f, "{:?}", chunk)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "Total opened: {}", self.total_opened)?;
        writeln!(f, "Max states: {}", self.max_states)?;
        writeln!(f, "Number of moves: {}", self.path.len() - 1)?;
        Ok(())
    }
}

fn zero(_p1: &Puzzle, _p2: &Puzzle) -> u32 {
    0
}

fn misplaced_tiles(p1: &Puzzle, p2: &Puzzle) -> u32 {
    let mut misplaced = 0;
    for i in 0..p1.flat.len() {
        if p1.flat[i] != p2.flat[i] {
            misplaced += 1;
        }
    }
    misplaced
}

// working only for idx + 1 (on flat) puzzle goal
fn manhattan_distance(p1: &Puzzle, _p2: &Puzzle) -> u32 {
    let mut distance: u32 = 0;
    for i in 0..p1.flat.len() {
        if p1.flat[i] == 0 {
            continue;
        }
        let goal_x = (p1.flat[i] - 1) as usize / p1.n;
        let goal_y = (p1.flat[i] - 1) as usize % p1.n;
        let x = i / p1.n;
        let y = i % p1.n;
        let dx = x as i16 - goal_x as i16;
        let dy = y as i16 - goal_y as i16;
        distance += (dx.abs() + dy.abs()) as u32;
    }
    distance
}

fn get_heuristic(
    heuristic: Option<i32>,
) -> Box<dyn Fn(&Puzzle, &Puzzle) -> u32> {
    let opt = heuristic.unwrap_or(2);
    match opt {
        0 => Box::new(zero),
        1 => Box::new(misplaced_tiles),
        2 => Box::new(manhattan_distance),
        _ => Box::new(manhattan_distance),
    }
}

pub fn solve(start: Puzzle, end: Puzzle) {
    let heuristic = get_heuristic(None);
    let solution = a_star(start, end, heuristic);

    match solution {
        Some(s) => print!("{}", s),
        None => println!("No solution !"),
    }
}
