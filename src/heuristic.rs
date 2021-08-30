use super::graph::LinearConflictGraph;
use super::puzzle::Puzzle;
use super::tile::Tile;

pub enum Heuristic {
    Zero,
    HammingDistance,
    ManhattanDistance,
    LinearConflict,
}

// pub struct Point {
//     x: usize,
//     y: usize,
// }

// pub struct Tile {
//     value: u16,
//     goal: Point,
//     pos: Point,
// }

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

fn manhattan_distance(p1: &Puzzle, p2: &Puzzle) -> u32 {
    let mut distance: u32 = 0;
    let mut map = vec![0; p1.flat.len()];
    for i in 0..p2.flat.len() {
        map[p2.flat[i] as usize] = i;
    }
    for i in 0..p1.flat.len() {
        if p1.flat[i] == 0 {
            continue;
        }

        let j = map[p1.flat[i] as usize];
        let x1 = i % p1.n;
        let x2 = j % p1.n;
        let y1 = i / p1.n;
        let y2 = j / p1.n;
        let dx = x1 as i16 - x2 as i16;
        let dy = y1 as i16 - y2 as i16;
        distance += (dx.abs() + dy.abs()) as u32;
    }
    distance
}

fn linear_col_conflicts(p1: &Puzzle, p2: &Puzzle, col: usize) -> u32 {
    let mut lc = 0;
    let mut lng = LinearConflictGraph::new();
    let mut map = vec![0; p1.flat.len()];
    for i in 0..p2.flat.len() {
        map[p2.flat[i] as usize] = i;
    }
    for row1 in 0..p1.n {
        for row2 in (row1 + 1)..p1.n {
            let tile1 = p1.flat[col + row1 * p1.n];
            if tile1 == 0 {
                continue;
            }
            let tile1_goal = map[tile1 as usize];
            let tile1_goal_col = tile1_goal % p1.n;
            let tile1_goal_row = tile1_goal / p1.n;
            if tile1_goal_col != col {
                continue;
            }
            let tile2 = p1.flat[col + row2 * p1.n];
            if tile2 == 0 {
                continue;
            }
            let tile2_goal = map[tile2 as usize];
            let tile2_goal_col = tile2_goal % p1.n;
            let tile2_goal_row = tile2_goal / p1.n;
            if tile1_goal_col != tile2_goal_col {
                continue;
            }
            if tile1_goal_row > tile2_goal_row {
                lng.push_conflict(tile1, tile2);
            }
        }
    }
    while lng.is_conflicts() == true {
        let tile = lng.most_conflicts();
        lng.remove_conflict_with(tile);
        lc += 1
    }
    lc
}

fn linear_row_conflicts(p1: &Puzzle, p2: &Puzzle, row: usize) -> u32 {
    let mut lc = 0;
    let mut lng = LinearConflictGraph::new();
    let mut map = vec![0; p1.flat.len()];
    for i in 0..p2.flat.len() {
        map[p2.flat[i] as usize] = i;
    }
    for col1 in 0..p1.n {
        for col2 in (col1 + 1)..p1.n {
            let tile1 = p1.flat[col1 + row * p1.n];
            if tile1 == 0 {
                continue;
            }
            let tile1_goal = map[tile1 as usize];
            let tile1_goal_col = tile1_goal % p1.n;
            let tile1_goal_row = tile1_goal / p1.n;
            if tile1_goal_row != row {
                continue;
            }
            let tile2 = p1.flat[col2 + row * p1.n];
            if tile2 == 0 {
                continue;
            }
            let tile2_goal = map[tile2 as usize];
            let tile2_goal_col = tile2_goal % p1.n;
            let tile2_goal_row = tile2_goal / p1.n;
            if tile1_goal_row != tile2_goal_row {
                continue;
            }
            if tile1_goal_col > tile2_goal_col {
                lng.push_conflict(tile1, tile2);
            }
        }
    }
    while lng.is_conflicts() == true {
        let tile = lng.most_conflicts();
        lng.remove_conflict_with(tile);
        lc += 1
    }
    println!("\nlc = {}", lc);
    lc
}

fn line_conflicts(p1: &Puzzle, p2: &Puzzle) -> u32 {
    let mut row_conflicts = 0;
    for row in 0..p1.n {
        row_conflicts += linear_row_conflicts(p1, p2, row);
    }
    let mut col_conflicts = 0;
    for col in 0..p1.n {
        col_conflicts += linear_col_conflicts(p1, p2, col);
    }
    row_conflicts + col_conflicts
}

// for top to bottom, left to right
// for col in 0..p1.n {
//     for row in 0..p1.n {
//         print!("{} ", p1.flat[col + row * p1.n]);
//     }
// }

// [https://medium.com/swlh/looking-into-k-puzzle-heuristics-6189318eaca2]
// [https://cse.sc.edu/~mgv/csce580sp15/gradPres/HanssonMayerYung1992.pdf]
//
fn linear_conflict(p1: &Puzzle, p2: &Puzzle) -> u32 {
    manhattan_distance(p1, p2) + line_conflicts(p1, p2) * 2
}

pub fn get_heuristic(
    heuristic: Heuristic,
) -> Box<dyn Fn(&Puzzle, &Puzzle) -> u32> {
    let h = match heuristic {
        Heuristic::Zero => zero,
        Heuristic::HammingDistance => hamming_distance,
        Heuristic::ManhattanDistance => manhattan_distance,
        Heuristic::LinearConflict => linear_conflict,
    };

    Box::new(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_manhattan() {
        let sum_8 = Puzzle::from_matrix(
            3,
            vec![vec![1, 2, 5], vec![3, 0, 6], vec![7, 4, 8]],
        );
        let sum_9 = Puzzle::from_matrix(
            3,
            vec![vec![2, 1, 3], vec![5, 4, 0], vec![6, 7, 8]],
        );
        let sum_10 = Puzzle::from_matrix(
            3,
            vec![vec![4, 2, 5], vec![1, 0, 6], vec![3, 8, 7]],
        );
        let goal_classic = Puzzle::from_matrix(
            3,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]],
        );

        assert_eq!(manhattan_distance(&sum_8, &goal_classic), 8);
        assert_eq!(manhattan_distance(&sum_9, &goal_classic), 9);
        assert_eq!(manhattan_distance(&sum_10, &goal_classic), 10);
    }

    #[test]
    fn test_snail_manhattan() {
        let sum_12 = Puzzle::from_matrix(
            3,
            vec![vec![1, 2, 5], vec![3, 0, 6], vec![7, 4, 8]],
        );
        let sum_11 = Puzzle::from_matrix(
            3,
            vec![vec![2, 1, 3], vec![5, 4, 0], vec![6, 7, 8]],
        );
        let sum_16 = Puzzle::from_matrix(
            3,
            vec![vec![4, 2, 5], vec![1, 0, 6], vec![3, 8, 7]],
        );
        let goal_snail = Puzzle::from_matrix(
            3,
            vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]],
        );

        assert_eq!(manhattan_distance(&sum_12, &goal_snail), 12);
        assert_eq!(manhattan_distance(&sum_11, &goal_snail), 11);
        assert_eq!(manhattan_distance(&sum_16, &goal_snail), 16);
    }

    #[test]
    fn test_big_classic_manhattan() {
        let sum_2 = Puzzle::from_matrix(
            4,
            vec![
                vec![2, 1, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 0],
            ],
        );
        let goal_classic = Puzzle::from_matrix(
            4,
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 0],
            ],
        );

        assert_eq!(manhattan_distance(&sum_2, &goal_classic), 2);
    }
    #[test]
    fn test_big_snail_manhattan() {
        let sum_4 = Puzzle::from_matrix(
            5,
            vec![
                vec![3, 2, 1, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 0, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9],
            ],
        );
        let goal_snail = Puzzle::from_matrix(
            5,
            vec![
                vec![1, 2, 3, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 0, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9],
            ],
        );

        assert_eq!(manhattan_distance(&sum_4, &goal_snail), 4);
    }

    #[test]
    fn test_linear_conflict() {
        let sum_1 = Puzzle::from_matrix(
            3,
            vec![vec![0, 2, 1], vec![7, 4, 5], vec![6, 3, 8]],
        );
        let sum_3 = Puzzle::from_matrix(
            3,
            vec![vec![0, 2, 1], vec![5, 4, 3], vec![6, 7, 8]],
        );
        let sum_5 = Puzzle::from_matrix(
            3,
            vec![vec![2, 7, 0], vec![5, 4, 3], vec![8, 1, 6]],
        );
        let goal_classic = Puzzle::from_matrix(
            3,
            vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
        );

        assert_eq!(line_conflicts(&sum_1, &goal_classic), 1);
        assert_eq!(line_conflicts(&sum_3, &goal_classic), 3);
        assert_eq!(line_conflicts(&sum_5, &goal_classic), 5);
    }
}
