use std::env;
use std::error::Error;
use std::path::Path;

mod a_star;
mod inversions;
mod parsing;
mod puzzle;
mod solution;
mod validity;

use puzzle::Puzzle;

fn usage() {
	println!("usage: ./n-puzzle [path_to_puzzle]");
}

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		usage();
		return Ok(());
	}

	let path = Path::new(&args[1]);
	if !path.is_file() {
		println!("Invalid file");
		return Ok(());
	}
	let (msize, matrix) = parsing::parse_puzzle(path)?;

	let puzzle = Puzzle::from_matrix(msize, matrix);
	if !validity::check_puzzle(&puzzle) {
		println!("Invalid puzzle");
		return Ok(());
	}

	solution::solve(puzzle);
	Ok(())
}
