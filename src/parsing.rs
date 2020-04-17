use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

type Matrix = Vec<Vec<u8>>;

fn read_file(filename: &Path) -> io::Result<Vec<String>> {
	let file = File::open(filename)?;
	let reader = BufReader::new(file);
	let lines = reader.lines();
	lines.collect()
}

fn sanitize_comments(lines: &Vec<String>) -> Vec<String> {
	lines
		.into_iter()
		.filter_map(|line| line.split("#").next())
		.filter(|new_line| new_line.len() != 0)
		.map(|s| s.to_string())
		.collect()
}

fn parse_puzzle(lines: &[String]) -> Result<Matrix, ParseIntError> {
	lines
		.into_iter()
		.map(|line| {
			line.split_whitespace()
				.map(|number| number.parse::<u8>())
				.collect::<Result<Vec<u8>, ParseIntError>>()
		})
		.collect::<Result<Matrix, ParseIntError>>()
}

pub fn parsing(filename: &Path) -> Result<(u8, Matrix), Box<dyn Error>> {
	let file_lines = read_file(filename)?;
	let puzzle_maybe = sanitize_comments(&file_lines);
	let size = puzzle_maybe[0].trim().parse::<u8>()?;
	let matrix = parse_puzzle(&puzzle_maybe[1..])?;
	Ok((size, matrix))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bad_path() {
		let path = Path::new("asldkfjasdlkfjas;dlfj");
		assert_eq!(parsing(path).is_err(), true);
	}

	#[test]
	fn test_subject_1() {
		let path = Path::new("./puzzles/subject-1.txt");
		assert_eq!(
			parsing(path).unwrap(),
			(3, vec![vec![3, 2, 6], vec![1, 4, 0], vec![8, 7, 5]])
		);
	}

	#[test]
	fn test_subject_2() {
		let path = Path::new("./puzzles/subject-2.txt");
		assert_eq!(
			parsing(path).unwrap(),
			(
				4,
				vec![
					vec![0, 10, 5, 7],
					vec![11, 14, 4, 8],
					vec![1, 2, 6, 13],
					vec![12, 3, 15, 9]
				]
			)
		);
	}

	#[test]
	fn test_subject_3() {
		let path = Path::new("./puzzles/subject-3.txt");
		assert_eq!(
			parsing(path).unwrap(),
			(
				4,
				vec![
					vec![0, 10, 5, 7],
					vec![11, 14, 4, 8],
					vec![1, 2, 6, 13],
					vec![12, 3, 15, 9]
				]
			)
		);
	}
}
