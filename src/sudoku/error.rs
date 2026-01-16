use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::sudoku::Grid;

#[derive(Debug)]
pub enum UniquenessConstraint {
    Row,
    Column,
    SubGrid,
}

#[derive(Debug)]
pub struct InvalidRow {
    pub row_id: usize,
}

#[derive(Debug)]
pub struct InvalidColumn {
    pub column_id: usize,
}

#[derive(Debug)]
pub struct InvalidSubGrid {
    pub subgrid_id: usize,
}

#[derive(Debug)]
pub struct UniquenessError {
    row: usize,
    column: usize,
    value: u8,
    violation: UniquenessConstraint,
}

#[derive(Debug)]
pub struct AnswerRangeError {
    value: u8,
}

impl InvalidRow {
    pub fn new(row_id: usize) -> Self {
        Self {row_id}
    }
}

impl InvalidColumn {
    pub fn new(column_id: usize) -> Self {
        Self {column_id}
    }
}

impl InvalidSubGrid {
    pub fn new(subgrid_id: usize) -> Self {
        Self {subgrid_id}
    }
}

impl UniquenessError {
    pub fn new(row: usize, column: usize, value: u8, violation: UniquenessConstraint) -> Self {
        Self {row, column, value, violation}
    }
}

impl AnswerRangeError {
    pub fn new(value: u8) -> Self {
        Self {value}
    }
}

impl Display for InvalidRow {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Invalid row id: {}", self.row_id)
    }
}

impl Display for InvalidColumn {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Invalid column id: {}", self.column_id)
    }
}

impl Display for InvalidSubGrid {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Invalid subgrid id: {}", self.subgrid_id)
    }
}

impl Display for UniquenessError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let violation = match self.violation {
            UniquenessConstraint::Row => "row",
            UniquenessConstraint::Column => "column",
            UniquenessConstraint::SubGrid => "subgrid",
        };

        write!(
            f,
            "Cannot insert value {} at {}, {} due to {} uniqueness constraints",
            self.value,
            self.row,
            self.column,
            violation,
        )
    }
}

impl Display for AnswerRangeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Cell value of {} is out of bounds: must be between {} and {}",
            self.value,
            Grid::MIN_VALID_VAL,
            Grid::MAX_VALID_VAL,
        )
    }
}

impl Error for InvalidRow {}
impl Error for InvalidColumn {}
impl Error for InvalidSubGrid {}
impl Error for UniquenessError {}
impl Error for AnswerRangeError {}