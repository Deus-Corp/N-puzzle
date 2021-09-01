extern crate clap;
use clap::{App, Arg};

use std::process;

use super::heuristic::Heuristic;
use super::puzzle::{Difficulty, Kind};
use super::solution::Algorithm;

#[derive(Debug)]
pub struct Sia {
    pub file: Option<String>,
    pub kind: Kind,
    pub size: usize,
    pub heuristic: Heuristic,
    pub difficulty: Difficulty,
    pub algorithm: Algorithm,
}

fn not_supported(arg: &str, option: &str) -> ! {
    println!(
        "ERROR: Argument not supported '{}' for '--{}' option",
        arg, option
    );
    println!("");
    println!("For more information try --help");

    process::exit(1)
}

fn clap_your_hands() -> Sia {
    let clap_app = App::new("42 project: N-Puzzle")
		.version("AMG v12 biturbo")
		.author("Rémi Pinoit <rpinoit@student.42.fr>")
		.about("The goal of this project is to smack some N-puzzles with some kind of A* search algorithm.")
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
				.help("Kind of the puzzle goal")
		)
		.arg(
			Arg::with_name("size")
				.short("s")
				.long("size")
				.takes_value(true)
				.value_name("NUMBER")
				.help("The N we talk about")
		)
		.arg(
			Arg::with_name("heuristic")
				.short("h")
				.long("heuristic")
				.takes_value(true)
				.value_name("ZERO|HAMMING|MANHATTAN|LINEAR")
				.help("Heuristic function used in A*")
		)
		.arg(
			Arg::with_name("difficulty")
				.short("d")
				.long("difficulty")
				.takes_value(true)
				.value_name("EASY|MEDIUM|HARD")
				.help("This is how much randomized puzzle will be")
		)
        .arg(
			Arg::with_name("algorithm")
				.short("a")
				.long("algo")
				.takes_value(true)
				.value_name("ASTAR|IDASTAR")
				.help("Algorithm used to resolve puzzle")
		);

    let matches = clap_app.get_matches();

    /* file option										*/
    let input_file = matches.value_of("file");
    let file = input_file.map(|f| f.to_string());
    /*													*/

    /* kind option										*/
    let input_kind = matches.value_of("kind").unwrap_or("CLASSIC");
    let kind = match input_kind {
        "CLASSIC" | "classic" => Kind::Classic,
        "SNAIL" | "snail" => Kind::Snail,
        "REVERSE" | "reverse" => Kind::Reverse,
        _ => not_supported(input_kind, "kind"),
    };
    /*													*/

    /* size option										*/
    let input_size = matches.value_of("size").unwrap_or("3");
    let size = match input_size.parse() {
        Ok(s) => s,
        _ => not_supported(input_size, "size"),
    };
    /*													*/

    /* heuristic option										*/
    let input_heuristic =
        matches.value_of("heuristic").unwrap_or("HAMMING");
    let heuristic = match input_heuristic {
        "ZERO" | "zero" => Heuristic::Zero,
        "HAMMING" | "hamming" => Heuristic::HammingDistance,
        "MANHATTAN" | "manhattan" => Heuristic::ManhattanDistance,
        "LINEAR" | "linear" => Heuristic::LinearConflicts,
        _ => not_supported(input_heuristic, "heuristic"),
    };
    /*													*/

    /* difficulty option								*/
    let input_difficulty =
        matches.value_of("difficulty").unwrap_or("EASY");
    let difficulty = match input_difficulty {
        "EASY" | "easy" => Difficulty::Easy,
        "MEDIUM" | "medium" => Difficulty::Medium,
        "HIGH" | "high" => Difficulty::Hard,
        _ => not_supported(input_difficulty, "difficulty"),
    };
    /*													*/

    /* algorithm option 								*/
    let input_algorithm = matches.value_of("algorithm").unwrap_or("ASTAR");
    let algorithm = match input_algorithm {
        "ASTAR" | "astar" => Algorithm::AStar,
        "IDASTAR" | "idastar" => Algorithm::IDAStar,
        _ => not_supported(input_algorithm, "algorithm"),
    };
    /*													*/

    Sia {
        file,
        kind,
        size,
        heuristic,
        difficulty,
        algorithm,
    }
}

pub fn parse_args() -> Sia {
    clap_your_hands()
}
