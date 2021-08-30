use super::puzzle::Puzzle;

pub struct TilePosition {
    pub col: usize,
    pub row: usize,
}

pub struct Tile {
    pub value: u16,
    pub pos: TilePosition,
    pub goal: TilePosition,
}

impl Tile {
    pub fn new(puzzle: &Puzzle, col: usize, row: usize) -> Tile {
        let value = puzzle.flat[col + row * puzzle.n];
        let goal_index = puzzle.end[value as usize];
        Tile {
            value,
            pos: TilePosition { col, row },
            goal: TilePosition {
                col: goal_index % puzzle.n,
                row: goal_index / puzzle.n,
            },
        }
    }

    pub fn is_in_row_conflict_with(&self, other: &Tile) -> bool {
        self.value != 0
            && other.value != 0
            && self.goal.row == self.pos.row
            && self.goal.row == other.goal.row
            && self.goal.col > other.goal.col
    }

    pub fn is_in_col_conflict_with(&self, other: &Tile) -> bool {
        self.value != 0
            && other.value != 0
            && self.goal.col == self.pos.col
            && self.goal.col == other.goal.col
            && self.goal.row > other.goal.row
    }
}
