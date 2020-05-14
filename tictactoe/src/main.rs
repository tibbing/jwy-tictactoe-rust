use ::std::*;

mod lib;
use crate::lib::board::Board;
use crate::lib::board::GOAL;
use crate::lib::board::MAX_SIZE;
use crate::lib::player::Player;

fn main() {
  println!("\n\n------------------------------------------------");
  println!("Starting new game...");
  println!(" · Input coordinates in the form X Y");
  println!(" · The board can expand to a maximum size of {0}.",MAX_SIZE);
  println!(" · First to {0} wins!",GOAL);
  println!("------------------------------------------------\n");

    let bricks = Vec::new();
    let mut _board: Board = Board {
        bricks,
        player: Player::PlayerX,
        size: 2, // Initial size
    };


    _board.nextplayer();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let pos: Vec<_> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().expect("Not an Integer!"))
                    .collect();

                let _pos_x: i32 = pos[0];
                let _pos_y: i32 = pos[1];

                _board.place(_pos_x, _pos_y);
                _board.display();
                if _board.is_winning_move(_pos_x, _pos_y) {
                    println!("{0} wins!", _board.player.as_str());
                    break;
                }
            }
            Err(error) => println!("error: {}", error),
        }

        _board.nextplayer();
    }

    println!("Game over!");
}
