use super::generate;
use super::moves::Move;

pub type Matrix = Vec<Vec<u16>>;

pub enum Kind {
    Classic,
    _Snail,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Puzzle {
    pub n: usize,
    pub flat: Vec<u16>,
    pub blank: usize,
}

impl Puzzle {
    pub fn get_index_of(flat: &Vec<u16>, tile: u16) -> usize {
        flat.iter()
            .position(|&n| n == tile)
            .expect("No blank, invalid puzzle !")
    }

    pub fn from_matrix(msize: usize, matrix: Matrix) -> Puzzle {
        let flat =
            matrix.iter().flat_map(|row| row.iter()).cloned().collect();
        let blank = Puzzle::get_index_of(&flat, 0);

        Puzzle {
            n: msize,
            flat,
            blank,
        }
    }

    pub fn new(kind: &Kind, size: usize) -> Puzzle {
        match kind {
            Kind::Classic => generate::new_classic(size),
            Kind::_Snail => generate::new_snail(size),
        }
    }

    pub fn new_randomized(kind: &Kind, size: usize) -> Puzzle {
        let mut puzzle = Puzzle::new(kind, size);
        generate::generate_randomized(&mut puzzle, 1000);
        puzzle
    }

    pub fn neighbors(&self) -> Vec<Puzzle> {
        let mut neighbors = vec![];
        let moves = Move::moves(self);

        for i in 0..moves.len() {
            let neighbor = self.new_state(&moves[i]);
            neighbors.push(neighbor);
        }
        neighbors
    }

    fn new_state(&self, m: &Move) -> Puzzle {
        let mut new = self.clone();

        m.apply(&mut new);
        new
    }
}

use std::fmt;

impl fmt::Debug for Puzzle {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(dest, "{}:\n", self.n)?;
        for chunk in self.flat.chunks(self.n) {
            write!(dest, "{:?}\n", chunk)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_to_flat() {
        assert_eq!(
            Puzzle::from_matrix(
                3,
                vec![vec![1, 0, 3], vec![1, 2, 3], vec![1, 2, 3],]
            ),
            Puzzle {
                n: 3,
                flat: vec![1, 0, 3, 1, 2, 3, 1, 2, 3],
                blank: 1
            }
        );

        assert_eq!(
            Puzzle::from_matrix(
                3,
                vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 0],]
            ),
            Puzzle {
                n: 3,
                flat: vec![1, 1, 1, 2, 2, 2, 3, 3, 0],
                blank: 8
            }
        );

        assert_eq!(
            Puzzle::from_matrix(
                4,
                vec![
                    vec![0, 4, 4, 4],
                    vec![3, 3, 3, 3],
                    vec![2, 2, 2, 2],
                    vec![1, 1, 1, 1],
                ]
            ),
            Puzzle {
                n: 4,
                flat: vec![0, 4, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1],
                blank: 0
            }
        );
    }

    #[test]
    fn test_new_classic() {
        assert_eq!(
            Puzzle::new(&Kind::Classic, 3),
            Puzzle {
                n: 3,
                flat: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
                blank: 8,
            }
        );

        assert_eq!(
            Puzzle::new(&Kind::Classic, 4),
            Puzzle {
                n: 4,
                flat: vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0
                ],
                blank: 15,
            }
        );

        assert_eq!(
            Puzzle::new(&Kind::Classic, 5),
            Puzzle {
                n: 5,
                flat: vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                    17, 18, 19, 20, 21, 22, 23, 24, 0
                ],
                blank: 24
            }
        );
    }

    #[test]
    fn test_neighbors_mid() {
        let p = Puzzle::from_matrix(
            3,
            vec![vec![1, 2, 3], vec![4, 0, 5], vec![6, 7, 8]],
        );

        assert_eq!(
            p.neighbors(),
            vec![
                Puzzle {
                    n: 3,
                    flat: vec![1, 0, 3, 4, 2, 5, 6, 7, 8],
                    blank: 1
                },
                Puzzle {
                    n: 3,
                    flat: vec![1, 2, 3, 4, 7, 5, 6, 0, 8],
                    blank: 7
                },
                Puzzle {
                    n: 3,
                    flat: vec![1, 2, 3, 0, 4, 5, 6, 7, 8],
                    blank: 3
                },
                Puzzle {
                    n: 3,
                    flat: vec![1, 2, 3, 4, 5, 0, 6, 7, 8],
                    blank: 5
                },
            ]
        );
    }

    #[test]
    fn test_neighbors_top_left() {
        let p = Puzzle::from_matrix(
            3,
            vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
        );

        assert_eq!(
            p.neighbors(),
            vec![
                Puzzle {
                    n: 3,
                    flat: vec![3, 1, 2, 0, 4, 5, 6, 7, 8],
                    blank: 3
                },
                Puzzle {
                    n: 3,
                    flat: vec![1, 0, 2, 3, 4, 5, 6, 7, 8],
                    blank: 1
                },
            ]
        );
    }

    #[test]
    fn test_neighbors_bot() {
        let p = Puzzle::from_matrix(
            3,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 0, 8]],
        );

        assert_eq!(
            p.neighbors(),
            vec![
                Puzzle {
                    n: 3,
                    flat: vec![1, 2, 3, 4, 0, 6, 7, 5, 8],
                    blank: 4
                },
                Puzzle {
                    n: 3,
                    flat: vec![1, 2, 3, 4, 5, 6, 0, 7, 8],
                    blank: 6
                },
                Puzzle {
                    n: 3,
                    flat: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
                    blank: 8
                }
            ]
        );
    }

    #[test]
    fn test_neighbors_bot_right_4() {
        let p = Puzzle::from_matrix(
            4,
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 0],
            ],
        );

        assert_eq!(
            p.neighbors(),
            vec![
                Puzzle {
                    n: 4,
                    flat: vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 13, 14, 15,
                        12,
                    ],
                    blank: 11
                },
                Puzzle {
                    n: 4,
                    flat: vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0,
                        15,
                    ],
                    blank: 14
                },
            ]
        );
    }
}
