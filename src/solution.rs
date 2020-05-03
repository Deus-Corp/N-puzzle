use super::a_star::a_star;
use super::puzzle::{Kind, Puzzle};

fn misplaced_tiles(p1: &Puzzle, p2: &Puzzle) -> u32 {
    let mut misplaced = 0;
    for i in 0..p1.flat.len() {
        if p1.flat[i] != p2.flat[i] {
            misplaced += 1;
        }
    }
    misplaced
}

fn get_heuristic() -> Box<dyn Fn(&Puzzle, &Puzzle) -> u32> {
    match 1 {
        _ => Box::new(misplaced_tiles),
    }
}

pub fn solve(puzzle: Puzzle) {
    let goal = Puzzle::new(Kind::Classic, puzzle.n);
    let heuristic = get_heuristic();
    let path = a_star(puzzle, goal, heuristic).unwrap();

    for i in (0..path.len()).rev() {
        let node = &path[i];
        println!("{:?}", node);
    }
    println!("End");
}
