mod a_star;
mod args;
mod generate;
mod graph;
mod heuristics;
mod ida_star;
mod inversions;
mod moves;
mod parsing;
mod puzzle;
mod solution;
mod tile;
mod validity;

use std::error::Error;
use std::path::Path;

use args::{parse_args, Sia};
use puzzle::Puzzle;
use solution::solve;

fn get_puzzle_from_file(f: &String) -> Result<Puzzle, Box<dyn Error>> {
	let puzzle_path = Path::new(f);
	if !puzzle_path.is_file() {
		return Err("Invalid file, the path is wrong !".into());
	}

	let (msize, matrix) = parsing::parse_puzzle(puzzle_path)?;
	let custom_puzzle = Puzzle::from_matrix(msize, matrix);

	Ok(custom_puzzle)
}

fn get_random_puzzle(options: &Sia) -> Puzzle {
	Puzzle::new_randomized(options.kind, options.difficulty, options.size)
}

fn get_puzzle(options: &Sia) -> Result<Puzzle, Box<dyn Error>> {
	let new_puzzle = match &options.file {
		Some(f) => get_puzzle_from_file(f)?,
		None => get_random_puzzle(options),
	};
	Ok(new_puzzle)
}

fn get_puzzle_goal(options: &Sia) -> Puzzle {
	Puzzle::new(options.kind, options.size)
}

fn n_puzzle(options: &Sia) -> Result<(), Box<dyn Error>> {
	let mut puzzle = get_puzzle(options)?;
	let goal = get_puzzle_goal(options);

	if !validity::check_puzzle(&puzzle, &goal) {
		return Err("Invalid puzzle !".into());
	}
	puzzle.set_goal(&goal);

	Ok(solve(puzzle, goal, options))
}

fn main() {
	let options = parse_args();

	if let Err(err) = n_puzzle(&options) {
		println!("ERROR: {}", err);
	};
}
