use sudoku_rust::sudoku::{
    Grid,
    Solver,
};

fn main() {
    // let mut grid = Grid::new();
    //
    // grid.set_cell(0, 0, 1)
    //     .set_cell(1, 1, 2)
    //     .set_cell(2, 2, 3)
    //     .set_cell(3, 3, 4)
    //     .set_cell(4, 4, 5)
    //     .set_cell(5, 5, 6)
    //     .set_cell(6, 6, 7)
    //     .set_cell(7, 7, 8)
    //     .set_cell(8, 8, 9);

    let grid = Grid::from_array([
        [Some(1), None, None, None, None, None, None, None, None],
        [None, Some(2), None, None, None, None, None, None, None],
        [None, None, Some(3), None, None, None, None, None, None],
        [None, None, None, Some(4), None, None, None, None, None],
        [None, None, None, None, Some(5), None, None, None, None],
        [None, None, None, None, None, Some(6), None, None, None],
        [None, None, None, None, None, None, Some(7), None, None],
        [None, None, None, None, None, None, None, Some(8), None],
        [None, None, None, None, None, None, None, None, Some(9)],
    ]);

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

    let solver = Solver::new(grid);
    solver.solve();
}
