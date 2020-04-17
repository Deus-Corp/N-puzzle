mod parsing;
use std::path::Path;

fn main() {
	let path = Path::new("./puzzles/subject-1.txt");
	let puzzle = parsing::parsing(path);
	println!("{:?}", puzzle);
}
