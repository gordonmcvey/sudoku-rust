use std::error::Error;
use crate::sudoku::grid::{Grid};
use crate::sudoku::option_finder::OptionFinder;
use crate::sudoku::reference::GridReference;

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

    fn find_solution(&self, solution: &mut Grid, row_id: usize, column_id: usize) -> Result<bool, Box<dyn Error>> {
        if row_id > Grid::GRID_COLUMNS - 1 {
            // If we've passed the end of the grid then we've succeeded in finding a solution
            Ok(true)
        } else if column_id > Grid::GRID_ROWS - 1 {
            // If we've passed the end of this row then move to the next one
            self.find_solution(solution, row_id + 1, 0)
        } else if solution.cell(&GridReference::from_numbers(row_id, column_id)?).is_some() {
            // If this cell already has a value, move on to the next one
            self.find_solution(solution, row_id, column_id + 1)
        } else {
            // Try each possible value in this cell then attempt to solve the rest of the puzzle
            let grid_ref = GridReference::from_numbers(row_id, column_id)?;
            let options = OptionFinder::find_for_cell(solution, &grid_ref)?;

            for option in options {
                if solution.set_cell(&grid_ref, option).is_ok()
                    && self.find_solution(solution, row_id, column_id + 1)? {
                    return Ok(true)
                } else {
                    solution.clear_cell(&grid_ref);
                }
            }

            // If we got here then we failed to solve the puzzle on this branch, either we'll have
            // to backtrack and try another option, or there are no more options and the puzzle is
            // not solvable
            Ok(false)
        }
    }
}
