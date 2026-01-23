use crate::sudoku::error::{InvalidColumn, InvalidRow, InvalidSubGrid};
use crate::sudoku::grid::Grid;
use std::error::Error;

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

trait SubgridRefValidator {

    fn validate_subgrid_id(subgrid_id: usize) -> Result<usize, InvalidSubGrid> {
        match subgrid_id {
            0..SubgridReference::SUBGRID_ID_LIMIT => Ok(subgrid_id),
            _ => Err(InvalidSubGrid::new(subgrid_id)),
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

#[derive(Debug)]
pub struct SubgridReference {
    subgrid: usize,
}

impl SubgridReference {
    const SUBGRID_ID_LIMIT: usize = (Grid::GRID_ROWS * Grid::GRID_COLUMNS)
        / (Grid::SUBGRID_ROWS * Grid::SUBGRID_COLUMNS);

    pub fn new(subgrid: usize) -> Result<Self, InvalidSubGrid> {
        let subgrid = Self::validate_subgrid_id(subgrid)?;
        Ok(SubgridReference { subgrid })
    }

    pub fn from_grid_ref(grid_ref: &GridReference) -> SubgridReference {
        let subgrid = ((grid_ref.row_ref().row() / Grid::SUBGRID_COLUMNS) * Grid::SUBGRID_COLUMNS)
            + (grid_ref.column_ref().column() / Grid::SUBGRID_ROWS);

        // As it's impossible to pass an invalid grid reference there's no need for further validation
        Self { subgrid }
    }

    pub fn subgrid(&self) -> usize {
        self.subgrid
    }
}

impl SubgridRefValidator for SubgridReference {}
