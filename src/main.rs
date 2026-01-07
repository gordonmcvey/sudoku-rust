use sudoku_rust::sudoku::{Grid, OptionFinder, Solver};

fn main() {
    // Puzzle from https://en.wikipedia.org/wiki/Sudoku
    let grid = Grid::from_array([
        [Some(5), Some(3), None, None, Some(7), None, None, None, None],
        [Some(6), None, None, Some(1), Some(9), Some(5), None, None, None],
        [None, Some(9), Some(8), None, None, None, None, Some(6), None],

        [Some(8), None, None, None, Some(6), None, None, None, Some(3)],
        [Some(4), None, None, Some(8), None, Some(3), None, None, Some(1)],
        [Some(7), None, None, None, Some(2), None, None, None, Some(6)],

        [None, Some(6), None, None, None, None, Some(2), Some(0), None],
        [None, None, None, Some(4), Some(1), Some(9), None, None, Some(5)],
        [None, None, None, None, Some(8), None, None, Some(7), Some(9)],
    ]);

    // for a in grid.grid.chunks(9) {
    //     println!("{:#?}", a);
    // }

    for row in 0..9 {
        print!("\t");
        for col in 0..9 {
            let raw_val = grid.cell(row, col);
            match raw_val {
                Some(val) => print!(" {} ", val.to_string()),
                None => print!(" - "),
            }
            if 2 == col % 3 && col < 8 {
                print!("|");
            }
        }
        println!();
        if 2 == row % 3 && row < 8 {
            println!("\t---------+---------+---------");
        }
    }

    // println!("{:?}", grid.row(0));
    // println!("{:?}", grid.row(1));
    // println!("{:?}", grid.row(2));
    // println!("{:?}", grid.row(3));
    // println!("{:?}", grid.row(4));
    // println!("{:?}", grid.row(5));
    // println!("{:?}", grid.row(6));
    // println!("{:?}", grid.row(7));
    // println!("{:?}", grid.row(8));
    // println!();

    // println!("{:?}", grid.col(0));
    // println!("{:?}", grid.col(1));
    // println!("{:?}", grid.col(2));
    // println!("{:?}", grid.col(3));
    // println!("{:?}", grid.col(4));
    // println!("{:?}", grid.col(5));
    // println!("{:?}", grid.col(6));
    // println!("{:?}", grid.col(7));
    // println!("{:?}", grid.col(8));
    // println!();

    // println!("{:?}", grid.subgrid(0));
    // println!("{:?}", grid.subgrid(1));
    // println!("{:?}", grid.subgrid(2));
    // println!("{:?}", grid.subgrid(3));
    // println!("{:?}", grid.subgrid(4));
    // println!("{:?}", grid.subgrid(5));
    // println!("{:?}", grid.subgrid(6));
    // println!("{:?}", grid.subgrid(7));
    // println!("{:?}", grid.subgrid(8));
    // println!();

    // println!("{:?}", grid.subgrid_at(0, 1));
    // println!("{:?}", grid.subgrid_at(1, 4));
    // println!("{:?}", grid.subgrid_at(2, 7));
    // println!("{:?}", grid.subgrid_at(3, 1));
    // println!("{:?}", grid.subgrid_at(4, 4));
    // println!("{:?}", grid.subgrid_at(5, 7));
    // println!("{:?}", grid.subgrid_at(6, 1));
    // println!("{:?}", grid.subgrid_at(7, 4));
    // println!("{:?}", grid.subgrid_at(8, 7));
    // println!();

    let solver = Solver::new(&grid);
    solver.solve();

    // println!("{:?}", OptionFinder::find_for_cell(&grid, 0,0));;
    // println!("{:?}", OptionFinder::find_for_cell(&grid, 0,2));;
}
