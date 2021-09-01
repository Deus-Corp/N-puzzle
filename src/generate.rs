use super::moves::Move;
use super::puzzle::Puzzle;

fn generate_classic(n: usize) -> Vec<u16> {
    let flat_len = n * n;
    let mut flat = Vec::with_capacity(flat_len);
    for i in 0..flat_len as u16 {
        flat.push(i + 1);
    }
    flat
}

pub fn new_classic(n: usize) -> Puzzle {
    let mut flat = generate_classic(n);
    let end = vec![0; flat.len()];
    let blank = flat.len() - 1;
    flat[blank] = 0;
    Puzzle {
        n,
        flat,
        end,
        blank,
        was: Move::Hold,
    }
}

pub fn new_reverse(n: usize) -> Puzzle {
    let mut p = new_classic(n);
    p.flat.reverse();

    p.blank = Puzzle::get_index_of(&p.flat, 0);
    p
}

// [https://www.geeksforgeeks.org/print-a-given-matrix-in-spiral-form/]
//
fn generate_snail(n: usize) -> Vec<u16> {
    let mut flat = vec![0; n * n];
    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = n;
    let mut end_col = n;
    let mut nb = 1;
    while start_row < end_row && start_col < end_col {
        for i in start_col..end_col {
            flat[start_row * n + i] = nb;
            nb += 1;
        }
        start_row += 1;
        for i in start_row..end_row {
            flat[i * n + end_col - 1] = nb;
            nb += 1;
        }
        end_col -= 1;
        for i in (start_col..end_col).rev() {
            flat[(end_row - 1) * n + i] = nb;
            nb += 1;
        }
        end_row -= 1;
        for i in (start_row..end_row).rev() {
            flat[i * n + start_col] = nb;
            nb += 1;
        }
        start_col += 1;
    }
    flat
}

pub fn new_snail(n: usize) -> Puzzle {
    let mut flat = generate_snail(n);
    let end = vec![0; flat.len()];
    let blank = Puzzle::get_index_of(&flat, (n * n) as u16);
    flat[blank] = 0;
    Puzzle {
        n,
        flat,
        end,
        blank,
        was: Move::Hold,
    }
}

use rand::Rng;

pub fn randomize(puzzle: &mut Puzzle, iterations: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let moves = Move::moves(puzzle);
        let rand = rng.gen::<u8>() % moves.len() as u8;

        moves[rand as usize].apply(puzzle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_snail() {
        assert_eq!(
            new_snail(3),
            Puzzle {
                n: 3,
                flat: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                end: vec![],
                blank: 4,
                was: Move::Hold,
            }
        );

        assert_eq!(
            new_snail(4),
            Puzzle {
                n: 4,
                flat: vec![
                    1, 2, 3, 4, 12, 13, 14, 5, 11, 0, 15, 6, 10, 9, 8, 7
                ],
                end: vec![],
                blank: 9,
                was: Move::Hold,
            }
        );

        assert_eq!(
            new_snail(5),
            Puzzle {
                n: 5,
                flat: vec![
                    1, 2, 3, 4, 5, 16, 17, 18, 19, 6, 15, 24, 0, 20, 7,
                    14, 23, 22, 21, 8, 13, 12, 11, 10, 9
                ],
                end: vec![],
                blank: 12,
                was: Move::Hold,
            }
        );
    }
}
