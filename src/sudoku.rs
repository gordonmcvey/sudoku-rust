#[derive(Debug)]
pub struct Grid {
    grid: [[Option<u8>;9];9],
}

impl Grid {
    const MAX_ROWS:usize = 9;
    const MAX_COLS:usize = 9;

    pub fn new() -> Self {
        Self {
            grid: [[None; Self::MAX_COLS]; Self::MAX_ROWS],
        }
    }

    pub fn from_array(array_grid: [[Option<u8>;9];9]) -> Self {
        Self {
            grid: array_grid,
        }
    }
    
    pub fn cell(&self, x: usize, y: usize) -> &Option<u8> {
        // @todo Range check here
        &self.grid[x][y]
    }

    pub fn row(&self, row_index: usize) -> &[Option<u8>] {
        // @todo Range check here
        &self.grid[row_index]
    }

    // @todo Implement getters for columns and subgrids

    pub fn set_cell(&mut self, x: usize, y: usize, value: u8) -> &mut Self {
        // @todo Range check here
        self.grid[x][y] = Some(value);
        self
    }
}
