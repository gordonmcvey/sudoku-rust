use std::error::Error;
use crate::sudoku::grid::Grid;

pub type OptionList = Vec<u8>;

#[derive(Debug)]
pub struct OptionFinder<> {
}

impl OptionFinder {

    pub fn find_for_cell(grid: &Grid, row_id: usize, column_id: usize) -> Result<OptionList, Box<dyn Error>> {
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

    fn build_used_list(grid: &Grid, row_id: usize, column_id: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut used_values: Vec<u8> = grid.row_values(row_id)?;

        used_values.extend(grid.col_values(column_id)?);
        used_values.extend(grid.subgrid_values_at(row_id, column_id)?);

        used_values.sort();
        used_values.dedup();

        Ok(used_values)
    }
}
