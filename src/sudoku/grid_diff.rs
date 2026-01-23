use std::fmt::{Display, Formatter, Result as FmtResult, Error as fmtError};
use colored::Colorize;
use crate::sudoku::grid::{Grid};
use crate::sudoku::reference::GridReference;

pub struct GridDiff<'a> {
    base: &'a Grid,
    current: &'a Grid,
}

impl<'a> GridDiff<'a> {
    pub fn new(base: &'a Grid, current: &'a Grid) -> GridDiff<'a> {
        GridDiff { base, current }
    }
}

impl<'a> Display for GridDiff<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut output = String::with_capacity(2048);

        for row in 0..Grid::GRID_COLUMNS {
            output.push_str("\t");
            for col in 0..Grid::GRID_ROWS {
                let grid_ref = GridReference::from_numbers(row, col).map_err(|_| fmtError)?;
                let raw_val = self.current.cell(&grid_ref);
                let cooked_val = match raw_val {
                    Some(val) => {
                        let prev_val = self.base.cell(&grid_ref);
                        if prev_val != raw_val {
                            format!(" {}", val.to_string().bright_green())
                        } else {
                            format!(" {}", val.to_string().white())
                        }
                    },
                    None => format!("{}", String::from(" -").blue()),
                };
                output.push_str(cooked_val.as_str());

                if Grid::SUBGRID_COLUMNS - 1 == col % Grid::SUBGRID_COLUMNS && col < Grid::GRID_COLUMNS - 1 {
                    output.push_str(format!("{}", String::from(" |").yellow()).as_str());
                }
            }

            output.push('\n');
            if Grid::SUBGRID_ROWS - 1 == row % Grid::SUBGRID_ROWS && row < Grid::GRID_ROWS - 1 {
                output.push_str(format!("{}", String::from("\t-------+-------+-------\n").yellow()).as_str());
            }
        }

        write!(f, "{}", output)
    }
}
