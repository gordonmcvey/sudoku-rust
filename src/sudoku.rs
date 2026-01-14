// @todo Implement the game
#[derive(Debug)]
pub struct Game {
    grid: Grid,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Grid {
    grid: Vec<Option<u8>>,
}

impl Grid {
    pub const GRID_WIDTH: usize = 9;
    pub const GRID_HEIGHT: usize = 9;
    const SUBGRID_WIDTH: usize = 3;
    const SUBGRID_HEIGHT: usize = 3;
    const SUBGRID_ID_LIMIT: usize = (Self::GRID_WIDTH * Self::GRID_HEIGHT) / (Self::SUBGRID_WIDTH * Self::SUBGRID_HEIGHT);

    const MIN_VALID_VAL: u8 = 1;
    const MAX_VALID_VAL: u8 = 9;

    pub fn new() -> Self {
        Self {
            grid: vec![None; Self::GRID_WIDTH * Self::GRID_WIDTH],
        }
    }

    pub fn from_array(array_grid: [[Option<u8>;Self::GRID_WIDTH];Self::GRID_HEIGHT]) -> Self {
        let mut grid = Vec::with_capacity(Self::GRID_WIDTH * Self::GRID_WIDTH);

        for row in array_grid.iter() {
            grid.extend_from_slice(row);
        }

        Self { grid }
    }

    pub fn cell(&self, row_id: usize, col_id: usize) -> Result<&Option<u8>, String> {
        let row_id = Self::validate_row_id(row_id)?;
        let col_id = Self::validate_col_id(col_id)?;

        Ok(&self.grid[row_id * Self::GRID_HEIGHT + col_id])
    }

    pub fn row(&self, row_id: usize) -> Result<&[Option<u8>], String> {
        let row_id = Self::validate_row_id(row_id)?;
        Ok(&self.grid[row_id * Self::GRID_HEIGHT..(row_id + 1) * Self::GRID_HEIGHT])
    }

    // @todo This is probably not the preferred way to extrapolate the columns and it returns a Vec
    // instead of an array slice
    pub fn col(&self, col_id: usize) -> Result<Vec<&Option<u8>>, String> {
        /*
         * As we're simulating the grid with a 1-dimensional array, extracting a "column" involves
         * fetching every nth element from the array where n is the width of the grid, and offsetting
         * by a column ID in the range 0 .. n - 1 such that (for a grid that's 9 elements wide)
         * column 0 equates to elements [0, 9, 18 ...], column 1 is [1, 10, 19 ...], column 3 is
         * [2, 11, 20 ...] and so on
         */
        let col_id = Self::validate_col_id(col_id)?;
        Ok(self.grid
            .iter()
            .skip(col_id)
            .step_by(Self::GRID_WIDTH)
            .collect::<Vec<&Option<u8>>>())
    }

    // @todo This is a pretty hacky POC and could use a refactor into something that handles
    // selecting the subslices more elegantly
    pub fn subgrid(&self, subgrid_id: usize) -> Result<Vec<Option<u8>>, String> {
        /*
         * As we're simulating the grid with a 1-dimensional array, a "subgrid" can be considered to
         * be 3 sub-slices of 3 elements each, comprising a total of the 9 elements that make up the
         * subgrid.
         *
         * The start of the first slice of each sub grid lies in column 0, 3, or 6, and row 0, 3, or
         * 6.  If we're using IDs of 0 .. 8 to refer to each subgrid, then that means that the first
         * slice of each subgrid starts at index 0, 3, 6, then skips to 27, 30, 33, and so on.  Each
         * slice is 3 elements long, and the entire subgrid is made up of the sub-slices starting in
         * the same column from the next two rows (so subgrid 0 consists of the array elements
         * [0, 1, 2, 9, 10, 11, 18, 19, 20], subgrid 4 would consist of the elements
         * [30, 31, 32, 39, 40, 41, 48, 49, 50], and so on)
         */
        let subgrid_id = Self::validate_subgrid_id(subgrid_id)?;
        let subgrid_col = subgrid_id * Self::SUBGRID_WIDTH % Self::GRID_WIDTH;
        let subgrid_row = (
            (subgrid_id * Self::GRID_HEIGHT) / (Self::GRID_HEIGHT * Self::SUBGRID_HEIGHT)
        ) * (Self::GRID_HEIGHT * Self::SUBGRID_HEIGHT);
        let subgrid_index = subgrid_col + subgrid_row;

        let mut subgrid:Vec<Option<u8>> = Vec::new();
        for row_start in 0 .. 3 {
            subgrid.extend_from_slice(&self.grid[subgrid_index + (9 * row_start) .. subgrid_index + 3 + (9 * row_start)]);
        }

        Ok(subgrid)
    }

    pub fn subgrid_at(&self, row_id: usize, col_id: usize) -> Result<Vec<Option<u8>>, String> {
        self.subgrid(Self::coordinates_to_subgrid(row_id, col_id))
    }

    pub fn row_values(&self, row_id: usize) -> Result<Vec<u8>, String> {
        let row: &[Option<u8>] = self.row(row_id)?;

        // Is it safe to use unwrap() here?
        Ok(row.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect())
    }

    pub fn col_values(&self, column_id: usize) -> Result<Vec<u8>, String> {
        let col: Vec<&Option<u8>> = self.col(column_id)?;

        // Is it safe to use unwrap here?
        Ok(col.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect())
    }

    pub fn subgrid_values(&self, subgrid_id: usize) -> Result<Vec<u8>, String> {
        let subgrid: Vec<Option<u8>> = self.subgrid(subgrid_id)?;

        // Is it safe to use unwrap here?
        Ok(subgrid.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect())
    }

    pub fn subgrid_values_at(&self, row_id: usize, col_id: usize) -> Result<Vec<u8>, String> {
        self.subgrid_values(Self::coordinates_to_subgrid(row_id, col_id))
    }

    pub fn set_cell(&mut self, row_id: usize, col_id: usize, value: u8) -> Result<&mut Self, String> {
        let row_id = Self::validate_row_id(row_id)?;
        let col_id = Self::validate_col_id(col_id)?;
        let value = Self::validate_cell_value(value)?;

        self.grid[row_id * Self::GRID_HEIGHT + col_id] = Some(value);

        if !self.row_is_unique(row_id)?
            || !self.col_is_unique(col_id)?
            || !self.subgrid_is_unique_at(row_id, col_id)?
        {
            self.clear_cell(row_id, col_id)?;
        }

        Ok(self)
    }

    pub fn clear_cell(&mut self, row_id: usize, col_id: usize) -> Result<&mut Self, String> {
        let row_id = Self::validate_row_id(row_id)?;
        let col_id = Self::validate_col_id(col_id)?;

        self.grid[row_id * Self::GRID_HEIGHT + col_id] = None;
        Ok(self)
    }

    fn row_is_unique(&self, row_id: usize) -> Result<bool, String> {
        Ok(Self::values_are_unique(&mut self.row_values(row_id)?))
    }

    fn col_is_unique(&self, col_id: usize) -> Result<bool, String> {
        Ok(Self::values_are_unique(&mut self.col_values(col_id)?))
    }

    fn subgrid_is_unique(&self, subgrid_id: usize) -> Result<bool, String> {
        Ok(Self::values_are_unique(&mut self.subgrid_values(subgrid_id)?))
    }

    fn subgrid_is_unique_at(&self, row_id: usize, col_id: usize) -> Result<bool, String> {
        Ok(Self::values_are_unique(&mut self.subgrid_values_at(row_id, col_id)?))
    }

    fn coordinates_to_subgrid(row_id: usize, col_id: usize) -> usize {
        ((row_id / Self::SUBGRID_HEIGHT) * Self::SUBGRID_HEIGHT) + (col_id / Self::SUBGRID_WIDTH)
    }

    fn values_are_unique(values: &mut [u8]) -> bool {
        values.sort();

        for index in 0 .. values.len() - 1 {
            if values[index] == values[index + 1] {
                return false;
            }
        }

        true
    }

    fn validate_row_id(row_id: usize) -> Result<usize, String> {
        match row_id {
            0..Self::GRID_HEIGHT => Ok(row_id),
            _ => Err(format!("Invalid row id: {}", row_id)),
        }
    }

    fn validate_col_id(col_id: usize) -> Result<usize, String> {
        match col_id {
            0..Self::GRID_WIDTH => Ok(col_id),
            _ => Err(format!("Invalid column id: {}", col_id)),
        }
    }

    fn validate_subgrid_id(col_id: usize) -> Result<usize, String> {
        match col_id {
            0..Self::SUBGRID_ID_LIMIT => Ok(col_id),
            _ => Err(format!("Invalid subgrid id: {}", col_id)),
        }
    }

    fn validate_cell_value(value: u8) -> Result<u8, String> {
        match value {
            Self::MIN_VALID_VAL..=Self::MAX_VALID_VAL => Ok(value),
            _ => Err(format!(
                "Cells must have a value between {} and {}, {} not allowed",
                Self::MIN_VALID_VAL,
                Self::MAX_VALID_VAL,
                value,
            ))
        }
    }
}

#[derive(Debug)]
pub struct Solver<'problem> {
    problem: &'problem Grid,
    solution: Option<Grid>,
}

impl<'problem> Solver<'problem> {
    pub fn new(problem: &'problem Grid) -> Self {
        Self {
            problem,
            solution: None,
        }
    }

    pub fn solve(&mut self) -> &mut Self {
        let mut solution = self.problem.clone();

        let solved = self.find_solution(&mut solution, 0, 0);
        match solved {
            Ok(true) => self.solution = Some(solution),
            Ok(false) => self.solution = None,
            Err(err) => panic!("{}", err),
        }

        self
    }

    pub fn get_solution(&self) -> &Option<Grid> {
        &self.solution
    }

    fn find_solution(&self, solution: &mut Grid, row_id: usize, column_id: usize) -> Result<bool, String> {
        if row_id > Grid::GRID_HEIGHT - 1 {
            // If we've passed the end of the grid then we've succeeded in finding a solution
            Ok(true)
        } else if column_id > Grid::GRID_WIDTH - 1 {
            // If we've passed the end of this row then move to the next one
            self.find_solution(solution, row_id + 1, 0)
        } else if solution.cell(row_id, column_id)?.is_some() {
            // If this cell already has a value, move on to the next one
            self.find_solution(solution, row_id, column_id + 1)
        } else {
            // Try each possible value in this cell then attempt to solve the rest of the puzzle
            let options = OptionFinder::find_for_cell(solution, row_id, column_id)?;

            for option in options {
                solution.set_cell(row_id, column_id, option)?;
                if solution.cell(row_id, column_id)?.is_some() && self.find_solution(solution, row_id, column_id + 1)? {
                    return Ok(true)
                } else {
                    solution.clear_cell(row_id, column_id)?;
                }
            }

            // If we got here then we failed to solve the puzzle on this branch, either we'll have
            // to backtrack and try another option, or there are no more options and the puzzle is
            // not solvable
            Ok(false)
        }
    }
}

// @todo Implement option finder logic
#[derive(Debug)]
pub struct OptionFinder<> {
}

impl OptionFinder {

    pub fn find_for_cell(grid: &Grid, row_id: usize, column_id: usize) -> Result<Vec<u8>, String> {
        // Early out: If this cell already has a value then it can't have any options
        if grid.cell(row_id, column_id)?.is_some() {
            return Ok(Vec::new());
        }

        let mut options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let used_vals = Self::build_used_list(grid, row_id, column_id)?;

        for value in used_vals.iter() {
            let found: Option<usize> = options.iter().position(|pos| pos == value);
            if let Some(index) = found {
                options.remove(index);
            }
        }

        Ok(options)
    }

    fn build_used_list(grid: &Grid, row_id: usize, column_id: usize) -> Result<Vec<u8>, String> {
        let mut used_values: Vec<u8> = grid.row_values(row_id)?;
        used_values.extend(grid.col_values(column_id)?);
        used_values.extend(grid.subgrid_values_at(row_id, column_id)?);

        used_values.sort();
        used_values.dedup();

        Ok(used_values)
    }
}
