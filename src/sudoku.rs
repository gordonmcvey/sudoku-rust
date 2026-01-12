// @todo Implement the game
#[derive(Debug)]
pub struct Game {
    grid: Grid,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Option<u8>>,
}

impl Grid {
    const MAX_ROWS:usize = 9;
    const MAX_COLS:usize = 9;

    pub fn new() -> Self {
        Self {
            grid: vec![None; Self::MAX_COLS * Self::MAX_COLS],
        }
    }

    pub fn from_array(array_grid: [[Option<u8>;9];9]) -> Self {
        let mut grid = Vec::with_capacity(Self::MAX_COLS * Self::MAX_COLS);

        for row in array_grid.iter() {
            grid.extend_from_slice(row);
        }

        Self { grid }
    }

    pub fn cell(&self, row_id: usize, col_id: usize) -> &Option<u8> {
        // @todo Range check here
        &self.grid[row_id * Self::MAX_ROWS + col_id]
    }

    pub fn row(&self, row_id: usize) -> &[Option<u8>] {
        // @todo Range check here
        &self.grid[row_id * Self::MAX_ROWS..(row_id + 1) * Self::MAX_ROWS]
    }

    // @todo This is probably not the preferred way to extrapolate the columns and it returns a Vec
    // instead of an array slice
    pub fn col(&self, col_id: usize) -> Vec<&Option<u8>> {
        // @todo Range check here
        /*
         * As we're simulating the grid with a 1-dimensional array, extracting a "column" involves
         * fetching every nth element from the array where n is the width of the grid, and offsetting
         * by a column ID in the range 0 .. n - 1 such that (for a grid that's 9 elements wide)
         * column 0 equates to elements [0, 9, 18 ...], column 1 is [1, 10, 19 ...], column 3 is
         * [2, 11, 20 ...] and so on
         */
        self.grid
            .iter()
            .skip(col_id)
            .step_by(Self::MAX_COLS)
            .collect::<Vec<&Option<u8>>>()
    }

    // @todo This is a pretty hacky POC and could use a refactor into something that handles
    // selecting the subslices more elegantly and it should return a Vec instead of an array slice
    pub fn subgrid(&self, subgrid_id: usize) -> Vec<Option<u8>> {
        // @todo Range check here
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
        let subgrid_col = subgrid_id * 3 % 9;
        let subgrid_row = ((subgrid_id * 9) / 27) * 27;
        let subgrid_index = subgrid_col + subgrid_row;

        // println!("Row: {}, Col: {}, Index: {}", subgrid_row, subgrid_col, subgrid_index);
        // println!("subgrid row 1 {:?}:", &self.grid[subgrid_index .. subgrid_index + 3]);
        // println!("subgrid row 2 {:?}:", &self.grid[subgrid_index + 9 .. subgrid_index + 3 + 9]);
        // println!("subgrid row 3 {:?}:", &self.grid[subgrid_index + 18 .. subgrid_index + 3 + 18]);

        let mut subgrid:Vec<Option<u8>> = Vec::new();
        for row_start in 0 .. 3 {
            subgrid.extend_from_slice(&self.grid[subgrid_index + (9 * row_start) .. subgrid_index + 3 + (9 * row_start)]);
        }

        // println!("return subgrid values: {:?}:", subgrid);

        subgrid
    }

    pub fn subgrid_at(&self, row_id: usize, col_id: usize) -> Vec<Option<u8>> {
        let subgrid_id = ((row_id / 3) * 3) + (col_id / 3);
        // println!("calculated subgrid ID for {}, {} is: {}", row_id, col_id, subgrid_id);
        self.subgrid(subgrid_id)
    }

    pub fn row_values(&self, row_id: usize) -> Vec<u8> {
        let row: &[Option<u8>] = self.row(row_id);

        // Is it safe to use unwrap() here?
        row.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect()
    }

    pub fn col_values(&self, column_id: usize) -> Vec<u8> {
        let col: Vec<&Option<u8>> = self.col(column_id);

        // Is it safe to use unwrap here?
        col.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect()
    }

    pub fn subgrid_values(&self, subgrid_id: usize) -> Vec<u8> {
        let subgrid: Vec<Option<u8>> = self.subgrid(subgrid_id);

        // Is it safe to use unwrap here?
        subgrid.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect()
    }

    pub fn subgrid_values_at(&self, row_id: usize, col_id: usize) -> Vec<u8> {
        let subgrid_id = ((row_id / 3) * 3) + (col_id / 3);

        self.subgrid_values(subgrid_id)
    }

    pub fn set_cell(&mut self, row_id: usize, col_id: usize, value: u8) -> &mut Self {
        // @todo Range check here
        self.grid[row_id * Self::MAX_ROWS + col_id] = Some(value);
        self
    }

    pub fn clear_cell(&mut self, row_id: usize, col_id: usize) -> &mut Self {
        // @todo Range check here
        self.grid[row_id * Self::MAX_ROWS + col_id] = None;
        self
    }
}

#[derive(Debug)]
pub struct Solver<'problem> {
    problem: &'problem Grid,
    solution: Option<Grid>,
    finder: OptionFinder<'problem>,
}

impl<'problem> Solver<'problem> {
    pub fn new(problem: &'problem Grid, finder: OptionFinder<'problem>) -> Self {
        Self {
            problem,
            solution: None,
            finder,
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
        } else if self.problem.cell(row_id, column_id).is_some() {
            // If this cell already has a value, move on to the next one
            println!("[{}, {}] is already filled", row_id, column_id);
            return self.find_solution(row_id, column_id + 1);
        } else {
            // @todo Find a valid solution for this cell
            let options = self.finder.find_for_cell(row_id, column_id);

            println!("Options for [{}, {}]: {:?}", row_id, column_id, options);

            return self.find_solution(row_id, column_id + 1);
        }

        return false;
    }
}

// @todo Implement option finder logic
#[derive(Debug)]
pub struct OptionFinder<'this_grid> {
    grid: &'this_grid Grid,
}

impl <'this_grid>OptionFinder<'this_grid> {
    pub fn new(grid: &'this_grid Grid) -> Self {
        Self { grid }
    }

    pub fn find_for_cell(&self, row_id: usize, column_id: usize) -> Vec<u8> {
        // Early out: If this cell already has a value then it can't have any options
        if self.grid.cell(row_id, column_id).is_some() {
            return Vec::new();
        }

        let mut options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let used_vals = Self::build_used_list(self.grid, row_id, column_id);

        // println!("{:?}", filtered);

        for value in used_vals.iter() {
            let found: Option<usize> = options.iter().position(|pos| pos == value);
            if let Some(index) = found {
                options.remove(index);
            }
        }

        options
    }

    fn build_used_list(grid: &Grid, row_id: usize, column_id: usize) -> Vec<u8> {
        let mut used_values: Vec<u8> = grid.row_values(row_id);
        used_values.extend(grid.col_values(column_id));
        used_values.extend(grid.subgrid_values_at(row_id, column_id));

        used_values.sort();
        used_values.dedup();

        used_values
    }
}
