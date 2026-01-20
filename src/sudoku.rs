use crate::sudoku::error::{*};
use crate::sudoku::grid::Grid;

pub mod error;
pub mod grid;
pub mod option_finder;
pub mod solver;

// @todo Implement the game
// #[derive(Debug)]
// pub struct Game {
//     grid: Grid,
// }

// pub struct GridValue {
//     value: u8,
// }
//
// impl GridValue {
//     const MIN_VALID_VAL: u8 = 1;
//     const MAX_VALID_VAL: u8 = 9;
//
//     pub fn new(value: u8) -> Result<Self, AnswerRangeError> {
//         Ok(Self { value: Self::validate_cell_value(value)? })
//     }
//
//     pub fn value(&self) -> u8 {
//         self.value
//     }
//
//     fn validate_cell_value(value: u8) -> Result<u8, AnswerRangeError> {
//         match value {
//             Self::MIN_VALID_VAL..=Self::MAX_VALID_VAL => Ok(value),
//             _ => Err(AnswerRangeError::new(value))
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Solver<'problem> {
//     problem: &'problem Grid,
//     solution: Option<Grid>,
// }
//
// impl<'problem> Solver<'problem> {
//     pub fn new(problem: &'problem Grid) -> Self {
//         Self {
//             problem,
//             solution: None,
//         }
//     }
//
//     pub fn solve(&mut self) -> &mut Self {
//         let mut solution = self.problem.clone();
//
//         let solved = self.find_solution(&mut solution, 0, 0);
//         match solved {
//             Ok(true) => self.solution = Some(solution),
//             Ok(false) => self.solution = None,
//             Err(err) => panic!("{}", err),
//         }
//
//         self
//     }
//
//     pub fn get_solution(&self) -> &Option<Grid> {
//         &self.solution
//     }
//
//     fn find_solution(&self, solution: &mut Grid, row_id: usize, column_id: usize) -> Result<bool, Box<dyn Error>> {
//         if row_id > Grid::GRID_COLUMNS - 1 {
//             // If we've passed the end of the grid then we've succeeded in finding a solution
//             Ok(true)
//         } else if column_id > Grid::GRID_ROWS - 1 {
//             // If we've passed the end of this row then move to the next one
//             self.find_solution(solution, row_id + 1, 0)
//         } else if solution.cell(row_id, column_id)?.is_some() {
//             // If this cell already has a value, move on to the next one
//             self.find_solution(solution, row_id, column_id + 1)
//         } else {
//             // Try each possible value in this cell then attempt to solve the rest of the puzzle
//             let options = OptionFinder::find_for_cell(solution, row_id, column_id)?;
//
//             for option in options {
//                 if solution.set_cell(row_id, column_id, option).is_ok()
//                     && self.find_solution(solution, row_id, column_id + 1)? {
//                     return Ok(true)
//                 } else {
//                     solution.clear_cell(row_id, column_id)?;
//                 }
//             }
//
//             // If we got here then we failed to solve the puzzle on this branch, either we'll have
//             // to backtrack and try another option, or there are no more options and the puzzle is
//             // not solvable
//             Ok(false)
//         }
//     }
// }

// #[derive(Debug)]
// pub struct OptionFinder<> {
// }
// 
// impl OptionFinder {
// 
//     pub fn find_for_cell(grid: &Grid, row_id: usize, column_id: usize) -> Result<Vec<u8>, Box<dyn Error>> {
//         // Early out: If this cell already has a value then it can't have any options
//         if grid.cell(row_id, column_id)?.is_some() {
//             return Ok(Vec::new());
//         }
// 
//         let mut options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
//         let used_vals = Self::build_used_list(grid, row_id, column_id)?;
// 
//         for value in used_vals.iter() {
//             let found: Option<usize> = options.iter().position(|pos| pos == value);
//             if let Some(index) = found {
//                 options.remove(index);
//             }
//         }
// 
//         Ok(options)
//     }
// 
//     fn build_used_list(grid: &Grid, row_id: usize, column_id: usize) -> Result<Vec<u8>, Box<dyn Error>> {
//         let mut used_values: Vec<u8> = grid.row_values(row_id)?;
// 
//         used_values.extend(grid.col_values(column_id)?);
//         used_values.extend(grid.subgrid_values_at(row_id, column_id)?);
// 
//         used_values.sort();
//         used_values.dedup();
// 
//         Ok(used_values)
//     }
// }
