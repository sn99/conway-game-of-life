use Cell;

pub fn glider_shape(grid: &mut Vec<Vec<Cell>>) {
    grid[6][6].switch();
    grid[7][7].switch();
    grid[7][8].switch();
    grid[8][6].switch();
    grid[8][7].switch();
}

pub fn tumbler_shape(grid: &mut Vec<Vec<Cell>>) {
    grid[5][5].switch();
    grid[5][6].switch();
    grid[5][10].switch();
    grid[5][11].switch();

    grid[6][5].switch();
    grid[6][7].switch();
    grid[6][9].switch();
    grid[6][11].switch();

    grid[7][5].switch();
    grid[7][7].switch();
    grid[7][9].switch();
    grid[7][11].switch();

    grid[8][7].switch();
    grid[8][9].switch();

    grid[9][6].switch();
    grid[9][7].switch();
    grid[9][9].switch();
    grid[9][10].switch();

    grid[10][6].switch();
    grid[10][7].switch();
    grid[10][9].switch();
    grid[10][10].switch();
}