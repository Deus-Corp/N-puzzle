use std::error::Error;
use std::path::Path;

mod parsing;
mod validity;

fn main() -> Result<(), Box<dyn Error>> {
	let path = Path::new("./puzzles/subject-1.txt");
	let (size, matrix) = parsing::parse_puzzle(path)?;
	if !validity::check_puzzle(size, &matrix) {
		println!("Invalid puzzle");
		return Ok(());
	}
	println!("{:?} {:?}", size, matrix);
	Ok(())
}
