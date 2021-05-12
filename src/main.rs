extern crate clap;
use clap::{App, Arg};

use std::error::Error;
use std::path::Path;

mod a_star;
mod generate;
mod heuristic;
mod inversions;
mod moves;
mod parsing;
mod puzzle;
mod solution;
mod validity;

use puzzle::{Kind, Puzzle};

struct Sia {
	pub file: Option<String>,
	pub kind: Kind,
	pub size: usize,
	//heuristic
	//difficulty
	//algo
}

fn clap_your_hands() -> Sia {
	let clap_app = App::new("42 project: N-Puzzle")
		.version("AMG v12 biturbo")
		.author("Rémi Pinoit <rpinoit@student.42.fr>")
		.about("The goal of this project is to smack some N-puzzles with some A* search algorithm.")
		.arg(
			Arg::with_name("file")
				.short("f")
				.long("file")
				.takes_value(true)
				.value_name("FILE")
				.help("File with custom puzzle"))
		.arg(
			Arg::with_name("kind")
				.short("k")
				.long("kind")
				.takes_value(true)
				.value_name("CLASSIC|SNAIL")
				.help("Kind of puzzle goal")
		)
		.arg(
			Arg::with_name("size")
				.short("s")
				.long("size")
				.takes_value(true)
				.value_name("NUMBER")
				.help("The N in question")
		);

	let matches = clap_app.get_matches();

	let input_file = matches.value_of("file");
	let file = input_file.map(|f| f.to_string());

	let input_kind = matches.value_of("kind").unwrap_or("classic");
	let kind = match input_kind {
		"SNAIL" | "snail" => Kind::_Snail,
		_ => Kind::Classic,
	};

	let input_size = matches.value_of("size").unwrap_or("3");
	let size = match input_size.parse() {
		Ok(s) => s,
		_ => 3,
	};

	Sia { file, kind, size }
}

fn get_puzzle(args: &Sia) -> Result<Puzzle, Box<dyn Error>> {
	if let Some(f) = &args.file {
		let puzzle_path = Path::new(&f);
		if !puzzle_path.is_file() {
			panic!("Invalid file, the path is wrong !");
		}

		let (msize, matrix) = parsing::parse_puzzle(puzzle_path)?;
		let custom_puzzle = Puzzle::from_matrix(msize, matrix);
		if !validity::check_puzzle(&custom_puzzle) {
			panic!("Invalid puzzle, unsolvable !");
		}

		return Ok(custom_puzzle);
	}

	let random_puzzle = Puzzle::new_randomized(&args.kind, args.size);
	Ok(random_puzzle)
}

fn main() {
	let args = clap_your_hands();

	if let Ok(puzzle) = get_puzzle(&args) {
		let goal = Puzzle::new(&args.kind, puzzle.n);
		solution::solve(puzzle, goal);
	}
}
