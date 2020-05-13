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
        self.size = cmp::max(cmp::max(x.abs(), y.abs()), self.size);
    }

    pub fn is_winning_move(&self, x: i32, y: i32) -> bool {
        fn check_vertical(bricks: Vec<Brick>, x: i32) -> bool {
            return check_line(
                bricks
                    .into_iter()
                    .filter(|brick| brick.x == x)
                    .map(|brick| brick.y)
                    .collect(),
                3,
            );
        }

        fn check_line(mut values: Vec<i32>, target: i8) -> bool {
            values.sort();

            let mut _count = 0;
            let mut _last_val = values[0];
            for _i in 0..values.len() {
                let _p = values[_i];
                _count = if _p == _last_val + 1 { _count + 1 } else { 0 };
                if _count == target - 1 {
                    return true;
                }
                _last_val = _p;
            }
            return false;
        }

        let player_bricks = self
            .bricks
            .clone()
            .into_iter()
            .filter(|brick| brick.player.as_str() == self.player.as_str())
            .collect();
        if check_vertical(player_bricks, x) {
            return true;
        }
        return false;
    }

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
