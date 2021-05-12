use super::puzzle::Puzzle;

pub enum Heuristic {
    _Zero,
    _HammingDistance,
    ManhattanDistance,
    _LinearConflict,
}

fn zero(_p1: &Puzzle, _p2: &Puzzle) -> u32 {
    0
}

fn hamming_distance(p1: &Puzzle, p2: &Puzzle) -> u32 {
    let mut misplaced = 0;
    for i in 0..p1.flat.len() {
        if p1.flat[i] != p2.flat[i] {
            misplaced += 1;
        }
    }
    misplaced
}

// working only for idx + 1 (on flat) puzzle goal
fn manhattan_distance(p1: &Puzzle, _p2: &Puzzle) -> u32 {
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

fn linear_conflict(_p1: &Puzzle, _p2: &Puzzle) -> u32 {
    0
}

pub fn get_heuristic(
    heuristic: Heuristic,
) -> Box<dyn Fn(&Puzzle, &Puzzle) -> u32> {
    let h = match heuristic {
        Heuristic::_Zero => zero,
        Heuristic::_HammingDistance => hamming_distance,
        Heuristic::ManhattanDistance => manhattan_distance,
        Heuristic::_LinearConflict => linear_conflict,
    };

    Box::new(h)
}
