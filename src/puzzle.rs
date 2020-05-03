pub type Matrix = Vec<Vec<u16>>;

pub enum Kind {
    Classic,
    _Snail,
}

pub enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Puzzle {
    pub n: usize,
    pub flat: Vec<u16>,
    pub f: u32,
    pub g: u32,
    pub h: u32,
    pub previous: Box<Option<Puzzle>>,
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
            f: 0,
            g: 0,
            h: 0,
            previous: Box::new(None),
        }
    }

    fn new_classic(n: usize) -> Puzzle {
        let flat_len = n * n;
        let mut flat = Vec::with_capacity(flat_len);
        for i in 0..flat_len as u16 {
            flat.push(i + 1);
        }
        flat[flat_len - 1] = 0;
        Puzzle {
            n,
            flat,
            f: 0,
            g: 0,
            h: 0,
            previous: Box::new(None),
        }
    }

    pub fn new(kind: Kind, size: usize) -> Puzzle {
        match kind {
            Kind::Classic => Puzzle::new_classic(size),
            _ => unimplemented!(),
        }
    }

    fn moves(&self) -> Vec<Move> {
        let blank = self.blank();
        let row = blank / self.n + 1;
        let column = blank % self.n + 1;

        let mut moves = vec![];
        // can't move up
        if !(row == 1) {
            moves.push(Move::UP);
        }
        // can't move down
        if !(row == self.n) {
            moves.push(Move::DOWN);
        }
        // can't move left
        if !(column == 1) {
            moves.push(Move::LEFT);
        }
        // can't move right
        if !(column == self.n) {
            moves.push(Move::RIGHT);
        }
        moves
    }

    fn swap_up(&mut self) {
        let blank = self.blank();
        let up = blank - self.n;
        //print!("|up {}|", up);

        self.flat.swap(blank, up);
    }

    fn swap_down(&mut self) {
        let blank = self.blank();
        let down = blank + self.n;
        //print!("|blank {} ; down {} {:?}|", blank, down, self);

        self.flat.swap(blank, down);
    }

    fn swap_left(&mut self) {
        let blank = self.blank();
        let left = blank - 1;
        //print!("|left {}|", left);

        self.flat.swap(blank, left);
    }

    fn swap_right(&mut self) {
        let blank = self.blank();
        let right = blank + 1;
        //print!("|right {}|", right);

        self.flat.swap(blank, right);
    }

    fn new_state(&self, m: &Move) -> Puzzle {
        let mut new = self.clone();

        match m {
            Move::UP => new.swap_up(),
            Move::DOWN => new.swap_down(),
            Move::LEFT => new.swap_left(),
            Move::RIGHT => new.swap_right(),
        };

        new
    }

    pub fn neighbors(&self) -> Vec<Box<Puzzle>> {
        let mut neighbors = vec![];
        let moves = self.moves();

        for i in 0..moves.len() {
            let neighbor = self.new_state(&moves[i]);
            neighbors.push(Box::new(neighbor));
        }
        neighbors
    }

    pub fn blank(&self) -> usize {
        self.flat
            .iter()
            .position(|&n| n == 0)
            .expect("No blank, invalid puzzle !")
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

use std::cmp::Ordering;

impl Ord for Puzzle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flat.cmp(&other.flat)
    }
}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        self.flat == other.flat
    }
}

impl Eq for Puzzle {}
