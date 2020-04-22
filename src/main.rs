use std::env;
use std::error::Error;
use std::path::Path;

mod inversions;
mod parsing;
mod validity;

fn main() -> Result<(), Box<dyn Error>> {
	let args = env::args().collect::<Vec<_>>();
	let path = Path::new(&args[1]);
	let (msize, matrix) = parsing::parse_puzzle(path)?;
	if !validity::check_puzzle(&matrix, msize) {
		println!("Invalid puzzle");
		return Ok(());
	}
	println!("{:?} {:?}", msize, matrix);
	Ok(())
}
