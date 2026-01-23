use std::error::Error;
use crate::sudoku::grid::{Grid};
use crate::sudoku::reference::GridReference;

pub type OptionList = Vec<u8>;

#[derive(Debug)]
pub struct OptionFinder<> {
}

impl OptionFinder {

    pub fn find_for_cell(grid: &Grid, grid_ref: &GridReference) -> Result<OptionList, Box<dyn Error>> {
        // Early out: If this cell already has a value then it can't have any options
        if grid.cell(&grid_ref).is_some() {
            return Ok(Vec::new());
        }

        let mut options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let used_vals = Self::build_used_list(grid, &grid_ref)?;

        for value in used_vals.iter() {
            let found: Option<usize> = options.iter().position(|pos| pos == value);
            if let Some(index) = found {
                options.remove(index);
            }
        }

        Ok(options)
    }

    fn build_used_list(grid: &Grid, grid_ref: &GridReference) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut used_values: Vec<u8> = grid.row_values(&grid_ref.row_ref());

        used_values.extend(grid.col_values(&grid_ref.column_ref()));
        used_values.extend(grid.subgrid_values_at(&grid_ref)?);

        used_values.sort();
        used_values.dedup();

        Ok(used_values)
    }
}
