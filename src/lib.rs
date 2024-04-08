pub enum GameOfLifeCell {
    Alive,
    Dead,
}

pub struct Board {
    cells: Vec<GameOfLifeCell>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn cell(x: usize, y: usize) -> GameOfLifeCell {
        cells[x * height + y]
    }
}
