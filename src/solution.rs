use super::a_star::a_star;
use super::heuristic::{get_heuristic, Heuristic};
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

pub fn solve(start: Puzzle, end: Puzzle) {
    let h = get_heuristic(Heuristic::ManhattanDistance);
    let solution = a_star(start, end, h);

    match solution {
        Some(s) => print!("{}", s),
        None => println!("No solution !"),
    }
}
