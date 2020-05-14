use ::std::*;

mod lib;
use crate::lib::board::Board;
use crate::lib::board::GOAL;
use crate::lib::board::MAX_SIZE;

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

    let mut _board = Board::new();
    _board.nextplayer();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let pos: Vec<_> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i8>().expect("Not an Integer!"))
                    .collect();

                let _pos_x: i8 = pos[0];
                let _pos_y: i8 = pos[1];

                if _board.is_taken(_pos_x, _pos_y) {
                    println!("Oops! That spot is already taken. Try another one.");
                } else {
                    _board.place(_pos_x, _pos_y);
                    _board.display();
                    if _board.is_winning_move(_pos_x, _pos_y) {
                        println!("{0} wins!", _board.player.as_str());
                        break;
                    }
                    _board.nextplayer();
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    println!("Game over!");
}
