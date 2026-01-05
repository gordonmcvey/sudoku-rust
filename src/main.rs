use sudoku_rust::sudoku::{
    Grid,
    Solver,
};

fn main() {
    let mut grid = Grid::new();

    // grid
    //     .set_cell(0, 0, 1)
    //     .set_cell(1, 0, 1)
    //     .set_cell(2, 0, 1)
    //     .set_cell(1, 1, 2)
    //     .set_cell(2, 2, 3)
    //     .set_cell(3, 3, 4)
    //     .set_cell(4, 4, 5)
    //     .set_cell(5, 5, 6)
    //     .set_cell(6, 6, 7)
    //     .set_cell(7, 7, 8)
    //     .set_cell(8, 8, 9)
    // ;

    let grid = Grid::from_array([
        [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)],
        [Some(10), None, Some(12), Some(13), None, Some(15), Some(16), None, Some(18)],
        [Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27)],
        [Some(28), Some(29), Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36)],
        [Some(37), None, Some(39), Some(40), None, Some(42), Some(43), None, Some(45)],
        [Some(46), Some(47), Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54)],
        [Some(55), Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), Some(63)],
        [Some(64), None, Some(66), Some(67), None, Some(69), Some(70), None, Some(72)],
        [Some(73), Some(74), Some(75), Some(76), Some(77), Some(78), Some(79), Some(80), Some(81)],
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

    println!("{:?}", grid.row(0));
    println!("{:?}", grid.row(1));
    println!("{:?}", grid.row(2));
    println!("{:?}", grid.row(3));
    println!("{:?}", grid.row(4));
    println!("{:?}", grid.row(5));
    println!("{:?}", grid.row(6));
    println!("{:?}", grid.row(7));
    println!("{:?}", grid.row(8));
    println!();

    println!("{:?}", grid.col(0));
    println!("{:?}", grid.col(1));
    println!("{:?}", grid.col(2));
    println!("{:?}", grid.col(3));
    println!("{:?}", grid.col(4));
    println!("{:?}", grid.col(5));
    println!("{:?}", grid.col(6));
    println!("{:?}", grid.col(7));
    println!("{:?}", grid.col(8));
    println!("{:?}", grid.col(9));
    println!();

    println!("{:?}", grid.subgrid(0));
    println!("{:?}", grid.subgrid(1));
    println!("{:?}", grid.subgrid(2));
    println!("{:?}", grid.subgrid(3));
    println!("{:?}", grid.subgrid(4));
    println!();

    println!("{:?}", grid.subgrid_at(0, 1));
    println!("{:?}", grid.subgrid_at(1, 4));
    println!("{:?}", grid.subgrid_at(2, 7));
    println!("{:?}", grid.subgrid_at(3, 1));
    println!("{:?}", grid.subgrid_at(4, 4));
    println!("{:?}", grid.subgrid_at(5, 7));
    println!("{:?}", grid.subgrid_at(6, 1));
    println!("{:?}", grid.subgrid_at(7, 4));
    println!("{:?}", grid.subgrid_at(8, 7));
    println!();

    let solver = Solver::new(grid);
    solver.solve();
}
