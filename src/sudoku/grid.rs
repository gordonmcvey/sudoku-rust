use crate::sudoku::error::*;
use crate::sudoku::reference::*;
use colored::Colorize;
use std::error::Error;
use std::fmt::{Display, Error as fmtError, Formatter, Result as FmtResult};

#[derive(Debug)]
#[derive(Clone)]
pub struct Grid {
    // @todo Use the CellValue struct instead of U8 once I'm a bit more familiar with moving/borrowing
    grid_data: Vec<Option<u8>>,
}

impl Grid {
    pub(crate) const GRID_ROWS: usize = 9;
    pub(crate) const GRID_COLUMNS: usize = 9;
    pub(crate) const SUBGRID_ROWS: usize = 3;
    pub(crate) const SUBGRID_COLUMNS: usize = 3;

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
                    this_grid.set_cell(
                        &GridReference::from_numbers(row, col)?,
                        &CellValue::new(val.unwrap())?,
                    )?;
                }
            }
        }

        Ok(this_grid)
    }

    pub fn cell(&self, grid_ref: &GridReference) -> &Option<u8> {
        &self.grid_data[grid_ref.to_index()]
    }

    pub fn row(&self, row_ref: &RowReference) -> &[Option<u8>] {
        let row = row_ref.row();
        &self.grid_data[row * Self::GRID_COLUMNS..(row + 1) * Self::GRID_COLUMNS]
    }

    // @todo This is probably not the preferred way to extrapolate the columns and it returns a Vec
    // instead of an array slice
    pub fn column(&self, column_ref: &ColumnReference) -> Vec<&Option<u8>> {
        /*
         * As we're simulating the grid with a 1-dimensional array, extracting a "column" involves
         * fetching every nth element from the array where n is the width of the grid, and offsetting
         * by a column ID in the range 0 .. n - 1 such that (for a grid that's 9 elements wide)
         * column 0 equates to elements [0, 9, 18 ...], column 1 is [1, 10, 19 ...], column 3 is
         * [2, 11, 20 ...] and so on
         */
        self.grid_data
            .iter()
            .skip(column_ref.column())
            // Stepping is to the same column on the next row
            .step_by(Self::GRID_ROWS)
            .collect::<Vec<&Option<u8>>>()
    }

    // @todo This is a pretty hacky POC and could use a refactor into something that handles
    // selecting the subslices more elegantly
    pub fn subgrid(&self, subgrid_ref: &SubgridReference) -> Vec<Option<u8>> {
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
        let subgrid_id = subgrid_ref.subgrid();
        let subgrid_col = subgrid_id * Self::SUBGRID_ROWS % Self::GRID_ROWS;
        let subgrid_row = (
            (subgrid_id * Self::GRID_COLUMNS) / (Self::GRID_COLUMNS * Self::SUBGRID_COLUMNS)
        ) * (Self::GRID_COLUMNS * Self::SUBGRID_COLUMNS);
        let subgrid_index = subgrid_col + subgrid_row;

        let mut subgrid:Vec<Option<u8>> = Vec::new();
        for row_start in 0 .. 3 {
            subgrid.extend_from_slice(&self.grid_data[subgrid_index + (9 * row_start) .. subgrid_index + 3 + (9 * row_start)]);
        }

        subgrid
    }

    pub fn subgrid_at(&self, grid_ref: &GridReference) -> Vec<Option<u8>> {
        self.subgrid(&SubgridReference::from_grid_ref(grid_ref))
    }

    pub fn row_values(&self, row_ref: &RowReference) -> Vec<u8> {
        let row: &[Option<u8>] = self.row(&row_ref);

        // Is it safe to use unwrap() here?
        row.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect()
    }

    pub fn col_values(&self, column_ref: &ColumnReference) -> Vec<u8> {
        let col: Vec<&Option<u8>> = self.column(column_ref);

        // Is it safe to use unwrap here?
        col.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect()
    }

    pub fn subgrid_values(&self, subgrid_ref: &SubgridReference) -> Vec<u8> {
        let subgrid: Vec<Option<u8>> = self.subgrid(&subgrid_ref);

        // Is it safe to use unwrap here?
        subgrid.iter()
            .filter(|row| row.is_some())
            .map(|row| row.unwrap())
            .collect()
    }

    pub fn subgrid_values_at(&self, grid_ref: &GridReference) -> Vec<u8> {
        self.subgrid_values(&SubgridReference::from_grid_ref(&grid_ref))
    }

    pub fn set_cell(&mut self, grid_ref: &GridReference, value: &CellValue) -> Result<&mut Self, Box<dyn Error>> {
        let value = value.value();
        let index = grid_ref.to_index();
        let old_value = self.grid_data[index].clone();

        self.grid_data[index] = Some(value);

        let validated = self.validate_uniqueness(&grid_ref);
        if validated.is_err() {
            self.grid_data[index] = old_value;
            return Err(validated.unwrap_err().into());
        }

        Ok(self)
    }

    pub fn clear_cell(&mut self, grid_ref: &GridReference) -> &mut Self {
        self.grid_data[grid_ref.to_index()] = None;
        self
    }

    fn validate_uniqueness(&self, grid_ref: &GridReference) -> Result<(), Box<dyn Error>> {
        let row_ref = grid_ref.row_ref();
        let column_ref = grid_ref.column_ref();

        if !self.row_is_unique(&row_ref) {
            // Is it safe to use unwrap() here?
            return Err(UniquenessError::new(
                row_ref.row(),
                column_ref.column(),
                self.cell(&grid_ref).unwrap(),
                UniquenessConstraint::Row,
            ).into());
        }

        if !self.col_is_unique(&column_ref) {
            // Is it safe to use unwrap() here?
            return Err(UniquenessError::new(
                row_ref.row(),
                column_ref.column(),
                self.cell(&grid_ref).unwrap(),
                UniquenessConstraint::Column,
            ).into());
        }

        if !self.subgrid_is_unique_at(grid_ref) {
            // Is it safe to use unwrap() here?
            return Err(UniquenessError::new(
                row_ref.row(),
                column_ref.column(),
                self.cell(&grid_ref).unwrap(),
                UniquenessConstraint::SubGrid,
            ).into());
        }

        Ok(())
    }

    fn row_is_unique(&self, row_ref: &RowReference) -> bool {
        let mut row_values = self.row_values(row_ref);
        Self::values_are_unique(&mut row_values)
    }

    fn col_is_unique(&self, column_ref: &ColumnReference) -> bool {
        let mut col_values = self.col_values(&column_ref);
        Self::values_are_unique(&mut col_values)
    }

    fn subgrid_is_unique(&self, subgrid_ref: &SubgridReference) -> bool {
        let mut subgrid_values = self.subgrid_values(&subgrid_ref);
        Self::values_are_unique(&mut subgrid_values)
    }

    fn subgrid_is_unique_at(&self, grid_ref: &GridReference) -> bool {
        let mut subgrid_values = self.subgrid_values_at(&grid_ref);
        Self::values_are_unique(&mut subgrid_values)
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
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut output = String::with_capacity(2048);

        for row in 0..Self::GRID_COLUMNS {
            output.push_str("\t");
            for col in 0..Self::GRID_ROWS {
                let raw_val = self.cell(&GridReference::from_numbers(row, col).map_err(|_| fmtError)?);
                let cooked_val = match raw_val {
                    Some(val) => format!(" {}", val.to_string().white()),
                    None => format!("{}", String::from(" -").blue()),
                };
                output.push_str(cooked_val.as_str());

                if Self::SUBGRID_COLUMNS - 1 == col % Self::SUBGRID_COLUMNS && col < Self::GRID_COLUMNS - 1 {
                    output.push_str(format!("{}", String::from(" |").yellow()).as_str());
                }
            }

            output.push('\n');
            if Self::SUBGRID_ROWS - 1 == row % Self::SUBGRID_ROWS && row < Self::GRID_ROWS - 1 {
                output.push_str(format!("{}", String::from("\t-------+-------+-------\n").yellow()).as_str());
            }
        }

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub struct CellValue {
    value: u8,
}

impl CellValue {
    pub(crate) const MIN_VALID_VAL: u8 = 1;
    pub(crate) const MAX_VALID_VAL: u8 = 9;

    pub fn new(value: u8) -> Result<Self, AnswerRangeError> {
        let value = Self::validate_cell_value(value)?;
        Ok(Self { value })
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    fn validate_cell_value(value: u8) -> Result<u8, AnswerRangeError> {
        match value {
            Self::MIN_VALID_VAL..=Self::MAX_VALID_VAL => Ok(value),
            _ => Err(AnswerRangeError::new(value))
        }
    }
}
