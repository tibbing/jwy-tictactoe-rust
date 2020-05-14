use ::std::*;
use std::str::FromStr;

mod lib;
use crate::lib::board::Board;
use crate::lib::board::MAX_SIZE;
use crate::lib::position::Position;

pub const GOAL: i8 = 5;

fn main() {
    println!("\n\n------------------------------------------------");
    println!("Starting new game...");
    println!(" · Input coordinates in the form X Y");
    println!(
        " · The board can expand to a maximum size of {0}.",
        MAX_SIZE
    );
    println!(" · First to {0} wins!", GOAL);
    println!("------------------------------------------------\n");

    let mut _board = Board::new(GOAL);
    _board.nextplayer();

    loop {
        let input = read_input::<Position>();

        match input {
            Ok(position) => {
                if _board.is_taken(position) {
                    println!("Oops! That spot is already taken. Try another one.");
                } else if !_board.is_within_board(position) {
                    println!(
                        "Oops! That spot is outside the board. Board max size is {0}. Try another one.",
                        MAX_SIZE
                    );
                } else {
                    _board.place(position);
                    _board.display();
                    if _board.is_winning_move(position) {
                        println!("{0} wins!", _board.player.as_str());
                        break;
                    }
                    _board.nextplayer();
                }
            }
            Err(e) => println!("Oops, I didn't understand that! {0}. Try again.", e),
        }
    }

    println!("Game over!");
}

fn read_input<Position: FromStr>() -> Result<Position, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Input cant be read: {}", e))
        .and_then(|_| {
            input
                .trim()
                .parse::<Position>()
                .map_err(|_| "Not a valid position".to_string())
        })
}
