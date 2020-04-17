mod parsing;
use std::path::Path;

fn main() {
	let path = Path::new("./puzzles/subject-1.txt");
	parsing::parsing(path);
}
