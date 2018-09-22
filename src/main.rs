extern crate conway_game_of_life;

extern crate termion;

use conway_game_of_life::*;

fn main() {
    let mut grid = vec![vec![Cell::new(false); 15]; 15];

    grid[6][6].switch();
    grid[7][7].switch();
    grid[7][8].switch();
    grid[8][6].switch();
    grid[8][7].switch();

    print_grid(&grid);

    update_grid(&mut grid);

    println!();

    print_grid(&grid);

    update_grid(&mut grid);

    println!();

    print_grid(&grid);

}
