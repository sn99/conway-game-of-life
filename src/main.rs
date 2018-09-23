extern crate conway_game_of_life;

extern crate termion;

use conway_game_of_life::*;

fn main() {
    let mut grid = vec![vec![Cell::new(false); 20]; 20];

    println!("====================\n\
    Welcome to conway's game of life\n\
    ====================\n\
    The Game of Life is not your typical computer game. It is a 'cellular automaton', and was invent\
    ed by Cambridge mathematician John Conway.\nThis game became widely known when it was mentioned \
    in an article published by Scientific American in 1970. It consists of a collection of cells whi\
    ch, based on a few mathematical rules, can live, die or multiply. Depending on the initial condi\
    tions, the cells form various patterns throughout the course of the game.\n\
    ====================");

    eprint!("\nThe shapes available are :\n====================\n1.glider\n2.tumbler\n=============\
    =======\nEnter choice : ");

    let mut temp_input = String::new();
    std::io::stdin().read_line(&mut temp_input).expect("Error in 'input_message' function !");

    if temp_input.trim() == "1" {
        shapes::glider_shape(&mut grid);
    }
    if temp_input.trim() == "2" {
        shapes::tumbler_shape(&mut grid);
    }

    loop {
        print_grid(&grid);

        eprint!("enter 'y' for next iteration of grid and any other to exit : ");

        let mut temp_input = String::new();
        std::io::stdin().read_line(&mut temp_input).expect("Error in 'input_message' function !");

        if temp_input.trim() == "y" || temp_input.trim() == "Y" {
            update_grid(&mut grid);
        } else {
            break;
        }
    }
}
