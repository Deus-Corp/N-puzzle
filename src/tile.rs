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
}
