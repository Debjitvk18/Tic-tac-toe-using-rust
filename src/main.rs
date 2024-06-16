use std::{cell, io};

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE]; //2D array of size 3 X 3

fn initialize_board() -> Board {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}
fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}
fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        let mut input = String::new();
        println!("Player {} input (row,col)", current_player);
        io::stdin().read_line(&mut input).expect("Error");
        println!("input={}", input);

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();
        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);

            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }
        println!("Invalid move!!!");
    }
}
fn check_winner(current_player: char, board: &Board) -> bool {
    //row wise winner check
    for row in 0..BOARD_SIZE {
        if board[row][0] == current_player
            && board[row][1] == current_player
            && board[row][2] == current_player
        {
            return true;
        }
    }
    //col wise winner check
    for col in 0..BOARD_SIZE {
        if board[0][col] == current_player
            && board[1][col] == current_player
            && board[2][col] == current_player
        {
            return true;
        }
    }
     //diagonal  wise winner check
    if (board[0][0] == current_player
        && board[1][1] == current_player
        && board[2][2] == current_player)
        || (board[0][2] == current_player
            && board[1][1] == current_player
            && board[2][0] == current_player)
    {
        return true;
    }
    return false;
}
fn check_draw(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if cell == &' ' {
                return false;
            }
        }
        println!();
    }
    return true;
}
fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current board: ");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        if check_winner(current_player, &board) {
            println!("Player {} is winner", current_player);
            break;
        }

        if check_draw(&board) {
            println!("The Game is draw");
        }
        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        }
    }
}
fn main() {
    println!("Welcome to tic tac toe game");
    play_game();
}
