use ::std::*;

mod lib;
use crate::lib::board::Board;
use crate::lib::player::Player;

fn main() {
    println!("Starting new game...");
    let bricks = Vec::new();
    let mut _board: Board = Board {
        bricks,
        player: Player::PlayerX,
        size: 2,
    };

    let mut count = 0u32;
    _board.nextplayer();

    loop {
        count += 1;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                // println!("Placing marker at {}", input);
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

        // if count == 6 {
        //     break;
        // }

        _board.nextplayer();
    }

    println!("Game over!");
}
