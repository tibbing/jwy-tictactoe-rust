use crate::lib::brick::Brick;
use crate::lib::player::Player;
use ::std::*;

#[derive(Clone)]
pub struct Board {
    pub bricks: Vec<Brick>,
    pub player: Player,
    pub size: i32,
}

impl Board {
    pub fn place(&mut self, x: i32, y: i32) {
        let _brick: Brick = Brick {
            x: x,
            y: y,
            player: self.player,
        };
        self.bricks.push(_brick);
        self.size = cmp::max(cmp::max(x.abs(), y.abs()), self.size)
    }

    pub fn is_winning_move(&self, x: i32, y: i32) {}

    fn check_vertical(bricks: Vec<Brick>, x: i32) {}

    pub fn nextplayer(&mut self) {
        if let Player::PlayerX = self.player {
            self.player = Player::PlayerO
        } else {
            self.player = Player::PlayerX
        }
        println!("{} turn:", self.player.as_str());
    }

    pub fn display(&self) {
        println!("");
        for _y in (-self.size..self.size + 1).rev() {
            for _x in -self.size..self.size + 1 {
                print!("{0} ", self.get_brick(_x, _y));
            }
            println!("");
        }
        // for brick in self.bricks.iter() {
        //     println!("{0} at {1},{2}", brick.player.as_str(), brick.x, brick.y);
        // }
    }

    pub fn get_brick(&self, x: i32, y: i32) -> &str {
        for brick in self.bricks.iter() {
            if brick.x == x && brick.y == y {
                return brick.player.as_str();
            }
        }
        return "·";
    }
}
