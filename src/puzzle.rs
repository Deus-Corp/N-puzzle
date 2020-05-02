pub type Matrix = Vec<Vec<u16>>;

pub struct Puzzle {
    pub n: usize,
    pub flat: Vec<u16>,
}

impl Puzzle {
    pub fn from_matrix(msize: usize, matrix: Matrix) -> Puzzle {
        Puzzle {
            n: msize,
            flat: matrix
                .iter()
                .flat_map(|row| row.iter())
                .cloned()
                .collect(),
        }
    }
}
