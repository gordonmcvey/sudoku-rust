use std::error::Error;
use crate::sudoku::error::{InvalidColumn, InvalidRow, InvalidSubGrid};
use crate::sudoku::grid::Grid;

trait RowRefValidator {
    fn validate_row_id(row_id: usize) -> Result<usize, InvalidRow> {
        match row_id {
            0..Grid::GRID_COLUMNS => Ok(row_id),
            _ => Err(InvalidRow::new(row_id)),
        }
    }
}

trait ColumnRefValidator {
    fn validate_column_id(column_id: usize) -> Result<usize, InvalidColumn> {
        match column_id {
            0..Grid::GRID_ROWS => Ok(column_id),
            _ => Err(InvalidColumn::new(column_id)),
        }
    }
}

#[derive(Debug)]
pub struct RowReference {
    row: usize,
}

impl RowReference {
    pub fn new(row: usize) -> Result<Self, InvalidRow> {
        let row = Self::validate_row_id(row)?;
        Ok(RowReference { row })
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

impl RowRefValidator for RowReference {}

#[derive(Debug)]
pub struct ColumnReference {
    column: usize,
}

impl ColumnReference {
    pub fn new(column: usize) -> Result<Self, InvalidColumn> {
        let column = Self::validate_column_id(column)?;
        Ok(ColumnReference { column })
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl ColumnRefValidator for ColumnReference {}

#[derive(Debug)]
pub struct GridReference {
    row_ref: RowReference,
    column_ref: ColumnReference,
}

impl GridReference {
    pub fn new(row: RowReference, column: ColumnReference) -> Self {
        GridReference { row_ref: row, column_ref: column }
    }

    pub fn from_numbers(row_id: usize, column_id: usize) -> Result<GridReference, Box<dyn Error>> {
        Ok(Self::new(
            RowReference::new(row_id)?,
            ColumnReference::new(column_id)?,
        ))
    }

    pub fn row_ref(&self) -> &RowReference {
        &self.row_ref
    }

    pub fn column_ref(&self) -> &ColumnReference {
        &self.column_ref
    }

    pub fn to_index(&self) -> usize {
        self.row_ref.row * Grid::GRID_COLUMNS + self.column_ref.column
    }
}
