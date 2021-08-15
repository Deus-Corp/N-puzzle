use super::puzzle::Puzzle;

use std::collections::HashMap;

pub enum Heuristic {
    Zero,
    HammingDistance,
    ManhattanDistance,
    LinearConflict,
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
// fn manhattan_distance(p1: &Puzzle, _p2: &Puzzle) -> u32 {
//     let mut distance: u32 = 0;
//     for i in 0..p1.flat.len() {
//         if p1.flat[i] == 0 {
//             continue;
//         }
//         let goal_x = (p1.flat[i] - 1) as usize / p1.n;
//         let goal_y = (p1.flat[i] - 1) as usize % p1.n;
//         let x = i / p1.n;
//         let y = i % p1.n;
//         let dx = x as i16 - goal_x as i16;
//         let dy = y as i16 - goal_y as i16;
//         distance += (dx.abs() + dy.abs()) as u32;
//     }
//     distance
// }

fn manhattan_distance(p1: &Puzzle, p2: &Puzzle) -> u32 {
    let mut distance: u32 = 0;
    let mut map = HashMap::new();
    for i in 0..p2.flat.len() {
        map.insert(p2.flat[i], i);
    }
    for i in 0..p1.flat.len() {
        if p1.flat[i] == 0 {
            continue;
        }

        let j = map.get(&p1.flat[i]).unwrap();
        let x1 = i / p1.n;
        let x2 = j / p1.n;
        let y1 = i % p1.n;
        let y2 = j % p1.n;
        let dx = x1 as i16 - x2 as i16;
        let dy = y1 as i16 - y2 as i16;
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
}
