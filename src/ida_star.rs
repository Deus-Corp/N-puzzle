use super::puzzle::Puzzle;
use super::solution::Solution;

pub fn ida_star(
    start: Puzzle,
    end: Puzzle,
    h: Box<dyn Fn(&Puzzle, &Puzzle) -> u32>,
) -> Option<Solution> {
    unimplemented!()
}
