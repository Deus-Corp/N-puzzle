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

// working only for idx + 1 (on flat) puzzle goal
fn manhattan_ditance(p1: &Puzzle, _p2: &Puzzle) -> u32 {
    let mut distance: u32 = 0;
    for i in 0..p1.flat.len() {
        if p1.flat[i] == 0 {
            continue;
        }
        let goal_x = (p1.flat[i] - 1) as usize / p1.n;
        let goal_y = (p1.flat[i] - 1) as usize % p1.n;
        let x = i / p1.n;
        let y = i % p1.n;
        let dx = x as i16 - goal_x as i16;
        let dy = y as i16 - goal_y as i16;
        distance += (dx.abs() + dy.abs()) as u32;
    }
    distance
}

fn get_heuristic(heuristic: Option<i32>) -> Box<dyn Fn(&Puzzle, &Puzzle) -> u32> {
    let opt = heuristic.unwrap_or(1);
    match opt {
        1 => Box::new(manhattan_ditance),
        _ => Box::new(misplaced_tiles),
    }
}

pub fn solve(puzzle: Puzzle) {
    let goal = Puzzle::new(Kind::Classic, puzzle.n);
    let heuristic = get_heuristic(None);
    let path = a_star(puzzle, goal, heuristic).unwrap();

    for i in (0..path.len()).rev() {
        let node = &path[i];
        println!("{:?}", node);
    }
    println!("Moves: {}", path.len() - 1);
    println!("End");
}
