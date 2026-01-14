use sudoku_rust::sudoku::{Grid, Solver};
use colored::Colorize;

fn main() {
    // Puzzle from https://en.wikipedia.org/wiki/Sudoku
    let grid = Grid::from_array([
        [Some(5), Some(3), None, None, Some(7), None, None, None, None],
        [Some(6), None, None, Some(1), Some(9), Some(5), None, None, None],
        [None, Some(9), Some(8), None, None, None, None, Some(6), None],

        [Some(8), None, None, None, Some(6), None, None, None, Some(3)],
        [Some(4), None, None, Some(8), None, Some(3), None, None, Some(1)],
        [Some(7), None, None, None, Some(2), None, None, None, Some(6)],

        [None, Some(6), None, None, None, None, Some(2), Some(8), None],
        [None, None, None, Some(4), Some(1), Some(9), None, None, Some(5)],
        [None, None, None, None, Some(8), None, None, Some(7), Some(9)],
    ]);

    // Puzzle from https://en.wikipedia.org/wiki/Sudoku_solving_algorithms
    //
    // NOTE: This puzzle is designed to be pathological for machine solving algorithms.  It does have
    // a solution, but it will take a while for a simple solver to find it, especially for the debug
    // build

    // let grid = Grid::from_array([
    //     [None, None, None, None, None, None, None, None, None],
    //     [None, None, None, None, None, Some(3), None, Some(8), Some(5)],
    //     [None, None, Some(1), None, Some(2), None, None, None, None],
    //
    //     [None, None, None, Some(5), None, Some(7), None, None, None],
    //     [None, None, Some(4), None, None, None, Some(1), None, None],
    //     [None, Some(9), None, None, None, None, None, None, None],
    //
    //     [Some(5), None, None, None, None, None, None, Some(7), Some(3)],
    //     [None, None, Some(2), None, Some(1), None, None, None, None],
    //     [None, None, None, None, Some(4), None, None, None, Some(9)],
    // ]);

    println!();
    print_grid(&grid, None);
    println!();

    let mut solver = Solver::new(&grid);
    let solution = solver.solve().get_solution();

    if let Some(solution) = solution {
        print_grid(&solution, Some(&grid));
    } else {
        println!("No solution found");
    }
    println!();
}

fn print_grid(grid: &Grid, base: Option<&Grid>) {
    for row in 0..Grid::GRID_HEIGHT {
        print!("\t");
        for col in 0..Grid::GRID_WIDTH {
            // @todo Handle result better
            let raw_val = grid.cell(row, col).unwrap();
            match raw_val {
                Some(val) => {
                    // @todo Handle result better
                    if base.is_some() && base.unwrap().cell(row, col).unwrap() != raw_val {
                        print!(" {}", val.to_string().bright_green())
                    } else {
                        print!(" {}", val.to_string().white())
                    }
                },
                None => print!("{}", String::from(" -").blue()),
            }

            if 2 == col % 3 && col < 8 {
                print!("{}", String::from(" |").yellow());
            }
        }
        println!();
        if 2 == row % 3 && row < 8 {
            println!("{}", String::from("\t-------+-------+-------").yellow());
        }
    }
}
