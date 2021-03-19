use super::puzzle::Puzzle;

pub enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Move {
    pub fn moves(puzzle: &Puzzle) -> Vec<Move> {
        let row = puzzle.blank / puzzle.n + 1;
        let column = puzzle.blank % puzzle.n + 1;
        let mut moves = vec![];
        // can't move up
        if !(row == 1) {
            moves.push(Move::UP);
        }
        // can't move down
        if !(row == puzzle.n) {
            moves.push(Move::DOWN);
        }
        // can't move left
        if !(column == 1) {
            moves.push(Move::LEFT);
        }
        // can't move right
        if !(column == puzzle.n) {
            moves.push(Move::RIGHT);
        }
        moves
    }

    pub fn apply(puzzle: &mut Puzzle, m: &Move) {
        match m {
            Move::UP => Move::swap_up(puzzle),
            Move::DOWN => Move::swap_down(puzzle),
            Move::LEFT => Move::swap_left(puzzle),
            Move::RIGHT => Move::swap_right(puzzle),
        };
    }

    // can replace by map with key as enum and value as offset
    fn swap_up(puzzle: &mut Puzzle) {
        let up = puzzle.blank - puzzle.n;
        puzzle.flat.swap(puzzle.blank, up);
        puzzle.blank = up;
    }

    fn swap_down(puzzle: &mut Puzzle) {
        let down = puzzle.blank + puzzle.n;
        puzzle.flat.swap(puzzle.blank, down);
        puzzle.blank = down;
    }

    fn swap_left(puzzle: &mut Puzzle) {
        let left = puzzle.blank - 1;
        puzzle.flat.swap(puzzle.blank, left);
        puzzle.blank = left;
    }

    fn swap_right(puzzle: &mut Puzzle) {
        let right = puzzle.blank + 1;
        puzzle.flat.swap(puzzle.blank, right);
        puzzle.blank = right;
    }
}
