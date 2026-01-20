use std::error::Error;
use crate::sudoku::error::{*};

#[derive(Debug)]
#[derive(Clone)]
pub struct Grid {
    grid_data: Vec<Option<u8>>,
}

impl Grid {
    pub const GRID_ROWS: usize = 9;
    pub const GRID_COLUMNS: usize = 9;
    const SUBGRID_ROWS: usize = 3;
    const SUBGRID_COLUMNS: usize = 3;
    const SUBGRID_ID_LIMIT: usize = (Self::GRID_ROWS * Self::GRID_COLUMNS) / (Self::SUBGRID_ROWS * Self::SUBGRID_COLUMNS);

    pub(crate) const MIN_VALID_VAL: u8 = 1;
    pub(crate) const MAX_VALID_VAL: u8 = 9;

    pub fn new() -> Self {
        Self {
            grid_data: vec![None; Self::GRID_ROWS * Self::GRID_COLUMNS],
        }
    }

    pub fn from_array(array_grid: [[Option<u8>;Self::GRID_ROWS];Self::GRID_COLUMNS]) -> Result<Self, Box<dyn Error>> {
        let mut this_grid = Self::new();
        let mut val: Option<u8>;

        for row in 0 .. Self::GRID_COLUMNS {
            for col in 0 .. Self::GRID_ROWS {
                val = array_grid[row][col];
                if val.is_some() {
                    // Is it safe to use unwrap() here?
                    this_grid.set_cell(row, col, val.unwrap())?;
                }
            }
        }

        Ok(this_grid)
    }

    pub fn cell(&self, row_id: usize, column_id: usize) -> Result<&Option<u8>, Box<dyn Error>> {
        let row_id = Self::validate_row_id(row_id)?;
        let column_id = Self::validate_column_id(column_id)?;

        Ok(&self.grid_data[row_id * Self::GRID_COLUMNS + column_id])
    }

    pub fn row(&self, row_id: usize) -> Result<&[Option<u8>], InvalidRow> {
        let row_id = Self::validate_row_id(row_id)?;
        Ok(&self.grid_data[row_id * Self::GRID_COLUMNS..(row_id + 1) * Self::GRID_COLUMNS])
    }

    // @todo This is probably not the preferred way to extrapolate the columns and it returns a Vec
    // instead of an array slice
    pub fn column(&self, column_id: usize) -> Result<Vec<&Option<u8>>, InvalidColumn> {
        let column_id = Self::validate_column_id(column_id)?;

        /*
         * As we're simulating the grid with a 1-dimensional array, extracting a "column" involves
         * fetching every nth element from the array where n is the width of the grid, and offsetting
         * by a column ID in the range 0 .. n - 1 such that (for a grid that's 9 elements wide)
         * column 0 equates to elements [0, 9, 18 ...], column 1 is [1, 10, 19 ...], column 3 is
         * [2, 11, 20 ...] and so on
         */
        Ok(self.grid_data
            .iter()
            .skip(column_id)
            // Stepping is to the same column on the next row
            .step_by(Self::GRID_ROWS)
            .collect::<Vec<&Option<u8>>>())
    }

    // @todo This is a pretty hacky POC and could use a refactor into something that handles
    // selecting the subslices more elegantly
    pub fn subgrid(&self, subgrid_id: usize) -> Result<Vec<Option<u8>>, InvalidSubGrid> {
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
        let subgrid_id = match Self::validate_subgrid_id(subgrid_id) {
            Ok(sg) => sg,
            Err(e) => return Err(e),
        };

        let subgrid_col = subgrid_id * Self::SUBGRID_ROWS % Self::GRID_ROWS;
        let subgrid_row = (
            (subgrid_id * Self::GRID_COLUMNS) / (Self::GRID_COLUMNS * Self::SUBGRID_COLUMNS)
        ) * (Self::GRID_COLUMNS * Self::SUBGRID_COLUMNS);
        let subgrid_index = subgrid_col + subgrid_row;

        let mut subgrid:Vec<Option<u8>> = Vec::new();
        for row_start in 0 .. 3 {
            subgrid.extend_from_slice(&self.grid_data[subgrid_index + (9 * row_start) .. subgrid_index + 3 + (9 * row_start)]);
        }

        Ok(subgrid)
    }

    pub fn subgrid_at(&self, row_id: usize, column_id: usize) -> Result<Vec<Option<u8>>, InvalidSubGrid> {
        self.subgrid(Self::coordinates_to_subgrid(row_id, column_id))
    }

    pub fn row_values(&self, row_id: usize) -> Result<Vec<u8>, InvalidRow> {
        let row: &[Option<u8>] = self.row(row_id)?;

        // Is it safe to use unwrap() here?
        Ok(row.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect())
    }

    pub fn col_values(&self, column_id: usize) -> Result<Vec<u8>, InvalidColumn> {
        let col: Vec<&Option<u8>> = self.column(column_id)?;

        // Is it safe to use unwrap here?
        Ok(col.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect())
    }

    pub fn subgrid_values(&self, subgrid_id: usize) -> Result<Vec<u8>, InvalidSubGrid> {
        let subgrid: Vec<Option<u8>> = self.subgrid(subgrid_id)?;

        // Is it safe to use unwrap here?
        Ok(subgrid.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect())
    }

    pub fn subgrid_values_at(&self, row_id: usize, column_id: usize) -> Result<Vec<u8>, InvalidSubGrid> {
        self.subgrid_values(Self::coordinates_to_subgrid(row_id, column_id))
    }

    pub fn set_cell(&mut self, row_id: usize, column_id: usize, value: u8) -> Result<&mut Self, Box<dyn Error>> {
        let row_id = Self::validate_row_id(row_id)?;
        let column_id = Self::validate_column_id(column_id)?;
        let value = Self::validate_cell_value(value)?;

        let index = row_id * Self::GRID_COLUMNS + column_id;
        let old_value = self.grid_data[index].clone();

        self.grid_data[index] = Some(value);

        let validated = self.validate_uniqueness(row_id, column_id);
        if validated.is_err() {
            self.grid_data[index] = old_value;
            return Err(validated.unwrap_err().into());
        }

        Ok(self)
    }

    pub fn clear_cell(&mut self, row_id: usize, column_id: usize) -> Result<&mut Self, Box<dyn Error>> {
        let row_id = Self::validate_row_id(row_id)?;
        let column_id = Self::validate_column_id(column_id)?;

        self.grid_data[row_id * Self::GRID_COLUMNS + column_id] = None;
        Ok(self)
    }

    fn validate_uniqueness(&self, row_id: usize, column_id: usize) -> Result<(), Box<dyn Error>> {
        if !self.row_is_unique(row_id)? {
            // Is it safe to use unwrap() here?
            return Err(UniquenessError::new(
                row_id,
                column_id,
                self.cell(row_id, column_id)?.unwrap(),
                UniquenessConstraint::Row,
            ).into());
        }

        if !self.col_is_unique(column_id)? {
            // Is it safe to use unwrap() here?
            return Err(UniquenessError::new(
                row_id,
                column_id,
                self.cell(row_id, column_id)?.unwrap(),
                UniquenessConstraint::Column,
            ).into());
        }

        if !self.subgrid_is_unique_at(row_id, column_id)? {
            // Is it safe to use unwrap() here?
            return Err(UniquenessError::new(
                row_id,
                column_id,
                self.cell(row_id, column_id)?.unwrap(),
                UniquenessConstraint::SubGrid,
            ).into());
        }

        Ok(())
    }

    fn row_is_unique(&self, row_id: usize) -> Result<bool, InvalidRow> {
        let mut row_values = self.row_values(row_id)?;
        Ok(Self::values_are_unique(&mut row_values))
    }

    fn col_is_unique(&self, column_id: usize) -> Result<bool, InvalidColumn> {
        let mut col_values = self.col_values(column_id)?;
        Ok(Self::values_are_unique(&mut col_values))
    }

    fn subgrid_is_unique(&self, subgrid_id: usize) -> Result<bool, InvalidSubGrid> {
        let mut subgrid_values = self.subgrid_values(subgrid_id)?;
        Ok(Self::values_are_unique(&mut subgrid_values))
    }

    fn subgrid_is_unique_at(&self, row_id: usize, column_id: usize) -> Result<bool, InvalidSubGrid> {
        let mut subgrid_values = self.subgrid_values_at(row_id, column_id)?;
        Ok(Self::values_are_unique(&mut subgrid_values))
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

    fn coordinates_to_subgrid(row_id: usize, column_id: usize) -> usize {
        ((row_id / Self::SUBGRID_COLUMNS) * Self::SUBGRID_COLUMNS) + (column_id / Self::SUBGRID_ROWS)
    }

    fn validate_row_id(row_id: usize) -> Result<usize, InvalidRow> {
        match row_id {
            0..Self::GRID_COLUMNS => Ok(row_id),
            _ => Err(InvalidRow::new(row_id)),
        }
    }

    fn validate_column_id(column_id: usize) -> Result<usize, InvalidColumn> {
        match column_id {
            0..Self::GRID_ROWS => Ok(column_id),
            _ => Err(InvalidColumn::new(column_id)),
        }
    }

    fn validate_subgrid_id(subgrid_id: usize) -> Result<usize, InvalidSubGrid> {
        match subgrid_id {
            0..Self::SUBGRID_ID_LIMIT => Ok(subgrid_id),
            _ => Err(InvalidSubGrid::new(subgrid_id)),
        }
    }

    fn validate_cell_value(value: u8) -> Result<u8, AnswerRangeError> {
        match value {
            Self::MIN_VALID_VAL..=Self::MAX_VALID_VAL => Ok(value),
            _ => Err(AnswerRangeError::new(value))
        }
    }
}
