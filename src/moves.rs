use super::puzzle::Puzzle;

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn moves(puzzle: &Puzzle) -> Vec<Move> {
        let row = puzzle.blank / puzzle.n + 1;
        let column = puzzle.blank % puzzle.n + 1;
        let mut moves = vec![];
        // can't move up
        if !(row == 1) {
            moves.push(Move::Up);
        }
        // can't move down
        if !(row == puzzle.n) {
            moves.push(Move::Down);
        }
        // can't move left
        if !(column == 1) {
            moves.push(Move::Left);
        }
        // can't move right
        if !(column == puzzle.n) {
            moves.push(Move::Right);
        }
        moves
    }

    pub fn apply(&self, puzzle: &mut Puzzle) {
        let swap_blank = |puzzle: &mut Puzzle, idx: usize| {
            puzzle.flat.swap(puzzle.blank, idx);
            puzzle.blank = idx;
        };

        match self {
            Move::Up => swap_blank(puzzle, puzzle.blank - puzzle.n),
            Move::Down => swap_blank(puzzle, puzzle.blank + puzzle.n),
            Move::Left => swap_blank(puzzle, puzzle.blank - 1),
            Move::Right => swap_blank(puzzle, puzzle.blank + 1),
        };
    }
}
