use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parsing(filename: &Path) -> io::Result<()> {
	let file = File::open(filename)?;
	let reader = BufReader::new(file);
	let puzzle = reader.lines();
	for line in puzzle {
		println!("{}", line?);
	}
	Ok(())
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
		assert_eq!(parsing(path).unwrap(), ());
	}
}
