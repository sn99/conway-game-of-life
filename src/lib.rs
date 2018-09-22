extern crate termion;

use termion::{color, style};

pub fn print_grid(grid: &Vec<Vec<Cell>>) {
    for line in grid {
        for cell in line {
            if cell.life {
                eprint!("{}O{}{} ", color::Fg(color::Blue), style::Bold, color::Fg(color::Reset));
            } else {
                eprint!("{}O{}{} ", color::Fg(color::Red), style::Bold, color::Fg(color::Reset));
            }
        }
        println!();
    }
}

pub fn update_grid(grid: &mut Vec<Vec<Cell>>) {
    let mut temp_grid = grid.clone();

    for row in 1..(grid[0].len() - 1) {
        for column in 1..(grid.len() - 1) {
            let mut neighbour_count = 0;

            if temp_grid[row - 1][column - 1].life {
                neighbour_count += 1;
            }
            if temp_grid[row - 1][column].life {
                neighbour_count += 1;
            }
            if temp_grid[row - 1][column + 1].life {
                neighbour_count += 1;
            }
            if temp_grid[row][column - 1].life {
                neighbour_count += 1;
            }
            if temp_grid[row][column + 1].life {
                neighbour_count += 1;
            }
            if temp_grid[row + 1][column - 1].life {
                neighbour_count += 1;
            }
            if temp_grid[row + 1][column].life {
                neighbour_count += 1;
            }
            if temp_grid[row + 1][column + 1].life {
                neighbour_count += 1;
            }
            temp_grid[row][column].neighbour = neighbour_count;
        }
    }

    for row in 1..(grid[0].len() - 1) {
        for column in 1..(grid.len() - 1) {
            if temp_grid[row][column].life {
                if temp_grid[row][column].neighbour <= 1 {
                    grid[row][column].switch();
                }
                if temp_grid[row][column].neighbour >= 4 {
                    grid[row][column].switch();
                }
            } else {
                if temp_grid[row][column].neighbour == 3 {
                    grid[row][column].switch();
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    life: bool,
    neighbour: i32,
}

impl Cell {
    pub fn new(initial: bool) -> Self {
        Cell {
            life: initial,
            neighbour: 0,
        }
    }
    pub fn switch(&mut self) {
        self.life = !self.life;
    }
}