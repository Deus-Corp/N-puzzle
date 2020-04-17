use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

type Puzzle = Vec<Vec<u8>>;

fn read_file(filename: &Path) -> io::Result<Vec<String>> {
	let file = File::open(filename)?;
	let reader = BufReader::new(file);
	let lines = reader.lines();
	return lines.collect();
}

fn sanitize_comments(lines: &mut Vec<String>) -> Vec<String> {
	lines
		.into_iter()
		.filter_map(|lines| lines.split("#").next())
		.filter(|new_line| new_line.len() != 0)
		.map(|s| s.to_string())
		.collect()
}

fn parse_puzzle(lines: Vec<String>) -> Result<Puzzle, Box<dyn Error>> {
	Ok(lines
		.into_iter()
		.skip(1)
		.map(|line| {
			line.split_whitespace()
				.map(|number| number.parse().unwrap())
				.collect()
		})
		.collect())
}

pub fn parsing(filename: &Path) -> Result<Puzzle, Box<dyn Error>> {
	let mut file_lines_with_comments = read_file(filename)?;
	let files_lines = sanitize_comments(&mut file_lines_with_comments);
	let puzzle = parse_puzzle(files_lines)?;
	Ok(puzzle)
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
			vec![vec![3, 2, 6], vec![1, 4, 0], vec![8, 7, 5]]
		);
	}
}
