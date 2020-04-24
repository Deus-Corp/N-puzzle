use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

type Matrix = Vec<Vec<u16>>;

fn read_file(path: &Path) -> io::Result<Vec<String>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let lines = reader.lines();
	lines.collect()
}

fn sanitize_comments(lines: &Vec<String>) -> Vec<String> {
	lines
		.iter()
		.filter_map(|line| line.split("#").next())
		.filter(|new_line| new_line.len() != 0)
		.map(|s| s.to_string())
		.collect()
}

fn parse_matrix_size(line: &String) -> Result<usize, ParseIntError> {
	line.trim().parse::<usize>()
}

fn parse_matrix(lines: &[String]) -> Result<Matrix, ParseIntError> {
	lines
		.iter()
		.map(|line| {
			line.split_whitespace()
				.map(|number| number.parse::<u16>())
				.collect::<Result<Vec<_>, _>>()
		})
		.collect::<Result<Matrix, _>>()
}

pub fn parse_puzzle(path: &Path) -> Result<(usize, Matrix), Box<dyn Error>> {
	let file_lines = read_file(path)?;
	let puzzle_maybe = sanitize_comments(&file_lines);
	let msize = parse_matrix_size(&puzzle_maybe[0])?;
	let matrix = parse_matrix(&puzzle_maybe[1..])?;
	Ok((msize, matrix))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bad_path() {
		let path = Path::new("asldkfjasdlkfjas;dlfj");
		assert_eq!(parse_puzzle(path).is_err(), true);
	}

	#[test]
	fn test_subject_1() {
		let path = Path::new("./puzzles/subject-1.txt");
		assert_eq!(
			parse_puzzle(path).unwrap(),
			(3, vec![vec![3, 2, 6], vec![1, 4, 0], vec![8, 7, 5]])
		);
	}

	#[test]
	fn test_subject_2() {
		let path = Path::new("./puzzles/subject-2.txt");
		assert_eq!(
			parse_puzzle(path).unwrap(),
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
			parse_puzzle(path).unwrap(),
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
