// @todo Implement the game
#[derive(Debug)]
pub struct Game {
    grid: Grid,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Option<u8>>>,
}

impl Grid {
    const MAX_ROWS:usize = 9;
    const MAX_COLS:usize = 9;

    pub fn new() -> Self {
        Self {
            //grid: [[None; Self::MAX_COLS]; Self::MAX_ROWS],
            grid: vec![vec![None; Self::MAX_COLS]; Self::MAX_ROWS],
        }
    }

    pub fn from_array(array_grid: [[Option<u8>;9];9]) -> Self {
        let mut grid = Vec::new();
        for row in array_grid.iter() {
            grid.push(row.to_vec());
        }

        Self { grid }
    }

    pub fn cell(&self, x: usize, y: usize) -> &Option<u8> {
        // @todo Range check here
        &self.grid[x][y]
    }

    pub fn row(&self, row_index: usize) -> &[Option<u8>] {
        // @todo Range check here
        &self.grid[row_index]
    }

    pub fn col(&self, col_index: usize) -> Vec<Option<u8>> {

        let mut a:Vec<Option<u8>> = Vec::new();

        for row in self.grid.iter() {
            a.push(row[col_index]);
        }
        a
    }

    // @todo Implement getters for subgrids

    pub fn set_cell(&mut self, x: usize, y: usize, value: u8) -> &mut Self {
        // @todo Range check here
        self.grid[x][y] = Some(value);
        self
    }
}

#[derive(Debug)]
pub struct Solver {
    problem:Grid,
    solution:Option<Grid>,
}

impl Solver {
    pub fn new(problem:Grid) -> Self {
        Self {
            problem,
            solution:None,
        }
    }

    pub fn solve(&self) -> &Self {
        self.find_solution(0, 0);
        &self
    }

    pub fn get_solution(&self) -> &Option<Grid> {
        &self.solution
    }

    fn find_solution(&self, row_id:usize, column_id:usize) -> bool {
        if row_id > 8 {
            // If we've passed the end of the grid then we've succeeded in finding a solution
            return true;
        } else if column_id > 8 {
            // If we've passed the end of this row then move to the next one
            return self.find_solution(row_id + 1, 0);
        } else if self.problem.grid[row_id][column_id].is_some() {
            // If this cell already has a value, move on to the next one
            println!("[{}, {}] is already filled", row_id, column_id);
            return self.find_solution(row_id, column_id + 1);
        } else {
            // @todo Find a valid solution for this cell
            println!("[{}, {}] can be filled", row_id, column_id);
            return self.find_solution(row_id, column_id + 1);
        }

        return false;
    }
}

// @todo Implement option finder logic
#[derive(Debug)]
pub struct OptionFinder {
}
