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

fn main() {
    let mut grid = Grid::new();
    let mut cell_val = String::new();

    grid.set_cell(0, 0, 1)
        .set_cell(1, 1, 2)
        .set_cell(2, 2, 3)
        .set_cell(3, 3, 4)
        .set_cell(4, 4, 5)
        .set_cell(5, 5, 6)
        .set_cell(6, 6, 7)
        .set_cell(7, 7, 8)
        .set_cell(8, 8, 9);

    for row in 0..9 {
        for col in 0..9 {
            let raw_val = grid.cell(row, col);
            match raw_val {
                Some(val) => cell_val = val.to_string(),
                None => cell_val = "-".to_string(),
            }
            print!(" {} ", cell_val);
        }
        println!();
    }
}
