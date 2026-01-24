use crate::sudoku::grid::Grid;
use crate::sudoku::reference::GridReference;
use std::fmt::{Debug, Formatter};

pub type OptionList = Vec<u8>;

pub trait OptionFinder {
    fn find_for_cell(&self, grid_ref: &GridReference) -> OptionList;
}

#[derive(Debug)]
pub struct StandardOptionFinder<'problem> {
    problem: &'problem Grid,
}

impl<'problem> StandardOptionFinder<'problem> {
    pub fn new(problem: &'problem Grid) -> StandardOptionFinder<'problem> {
        Self { problem }
    }

    fn build_used_list(&self, grid_ref: &GridReference) -> OptionList {
        let mut used_values: OptionList = self.problem.row_values(&grid_ref.row_ref());

        used_values.extend(self.problem.col_values(&grid_ref.column_ref()));
        used_values.extend(self.problem.subgrid_values_at(&grid_ref));

        used_values.sort();
        used_values.dedup();

        used_values
    }
}

impl<'problem> OptionFinder for StandardOptionFinder<'problem> {
    fn find_for_cell(&self, grid_ref: &GridReference) -> OptionList {
        // Early out: If this cell already has a value then it can't have any options
        if self.problem.cell(&grid_ref).is_some() {
            return Vec::new();
        }

        let mut options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let used_vals = self.build_used_list(&grid_ref);

        for value in used_vals.iter() {
            let found: Option<usize> = options.iter().position(|pos| pos == value);
            if let Some(index) = found {
                options.remove(index);
            }
        }

        options
    }
}

impl Debug for dyn OptionFinder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Option Finder (dynamic)")
    }
}
