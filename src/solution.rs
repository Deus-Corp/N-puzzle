use super::a_star::a_star;
use super::args::Sia;
use super::heuristic;
use super::ida_star::ida_star;
use super::puzzle::Puzzle;

#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    AStar,
    IDAStar,
}

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
            writeln!(f, "{:?}", puzzle.was.opposite())?;
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

pub fn solve(start: Puzzle, end: Puzzle, options: &Sia) {
    let h = heuristic::get_heuristic(options.heuristic);

    let solution = match options.algorithm {
        Algorithm::AStar => a_star(start, end, h),
        Algorithm::IDAStar => ida_star(start, end, h),
    };

    match solution {
        Some(s) => print!("{}", s),
        None => println!("No solution !"),
    };
}
