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

    let row_count = grid[0].len();
    let column_count = grid.len();

// Upper left Cell
    {
        let mut neighbour_count = 0;
        if temp_grid[0][1].life {
            neighbour_count += 1;
        }
        if temp_grid[1][0].life {
            neighbour_count += 1;
        }
        if temp_grid[1][1].life {
            neighbour_count += 1;
        }
        temp_grid[0][0].neighbour = neighbour_count;
    }
// Upper right Cell
    {
        let mut neighbour_count = 0;
        if temp_grid[0][column_count - 2].life {
            neighbour_count += 1;
        }
        if temp_grid[1][column_count - 2].life {
            neighbour_count += 1;
        }
        if temp_grid[1][column_count - 1].life {
            neighbour_count += 1;
        }
        temp_grid[0][column_count - 1].neighbour = neighbour_count;
    }
// Lower left Cell
    {
        let mut neighbour_count = 0;
        if temp_grid[row_count - 1][1].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 2][0].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 2][1].life {
            neighbour_count += 1;
        }
        temp_grid[row_count - 1][0].neighbour = neighbour_count;
    }
// Lower right Cell
    {
        let mut neighbour_count = 0;
        if temp_grid[row_count - 1][column_count - 2].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 2][column_count - 2].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 2][column_count - 1].life {
            neighbour_count += 1;
        }
        temp_grid[row_count - 1][column_count - 1].neighbour = neighbour_count;
    }

// For upper Row
    for i in 1..(column_count - 1) {
        let mut neighbour_count = 0;
        if temp_grid[1][i - 1].life {
            neighbour_count += 1;
        }
        if temp_grid[1][i].life {
            neighbour_count += 1;
        }
        if temp_grid[1][i + 1].life {
            neighbour_count += 1;
        }
        if temp_grid[0][i - 1].life {
            neighbour_count += 1;
        }
        if temp_grid[0][i + 1].life {
            neighbour_count += 1;
        }
        temp_grid[0][i].neighbour = neighbour_count;
    }
// For lower Row
    for i in 1..(column_count - 1) {
        let mut neighbour_count = 0;
        if temp_grid[row_count - 2][i - 1].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 2][i].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 2][i + 1].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 1][i - 1].life {
            neighbour_count += 1;
        }
        if temp_grid[row_count - 1][i + 1].life {
            neighbour_count += 1;
        }
        temp_grid[row_count - 1][i].neighbour = neighbour_count;
    }
// For right Column
    for i in 1..(row_count - 1) {
        let mut neighbour_count = 0;
        if temp_grid[i - 1][1].life {
            neighbour_count += 1;
        }
        if temp_grid[i][1].life {
            neighbour_count += 1;
        }
        if temp_grid[i + 1][1].life {
            neighbour_count += 1;
        }
        if temp_grid[i - 1][0].life {
            neighbour_count += 1;
        }
        if temp_grid[i + 1][0].life {
            neighbour_count += 1;
        }
        temp_grid[i][0].neighbour = neighbour_count;
    }
// For left Column
    for i in 1..(row_count - 1) {
        let mut neighbour_count = 0;
        if temp_grid[i - 1][column_count - 2].life {
            neighbour_count += 1;
        }
        if temp_grid[i][column_count - 2].life {
            neighbour_count += 1;
        }
        if temp_grid[i + 1][column_count - 2].life {
            neighbour_count += 1;
        }
        if temp_grid[i + 1][column_count - 1].life {
            neighbour_count += 1;
        }
        if temp_grid[i - 1][column_count - 1].life {
            neighbour_count += 1;
        }
        temp_grid[i][column_count - 2].neighbour = neighbour_count;
    }

    for row in 1..(row_count - 1) {
        for column in 1..(column_count - 1) {
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

    for row in 0..row_count {
        for column in 0..column_count {
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