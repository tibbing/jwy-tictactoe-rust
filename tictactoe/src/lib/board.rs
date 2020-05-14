use crate::lib::brick::Brick;
use crate::lib::player::Player;
use ::std::*;

pub const INIT_SIZE: i8 = 2;
pub const MAX_SIZE: i8 = 15;
pub const EMPTY_CHAR: &str = "·";
pub const FIRST_PLAYER: Player = Player::PlayerX;

#[derive(Clone)]
pub struct Board {
    pub bricks: Vec<Brick>,
    pub player: Player,
    pub size: i8,
    pub goal: i8,
}

impl Board {
    pub fn new(goal: i8) -> Board {
        Board {
            bricks: Vec::new(),
            player: FIRST_PLAYER,
            size: INIT_SIZE,
            goal,
        }
    }

    pub fn place(&mut self, x: i8, y: i8) {
        let _brick: Brick = Brick {
            x: x,
            y: y,
            player: self.player,
        };
        self.bricks.push(_brick);
        self.size = cmp::max(cmp::max(x.abs(), y.abs()), self.size);
    }

    pub fn is_winning_move(&self, x: i8, y: i8) -> bool {
        fn check_diagonal(bricks: Vec<Brick>, goal: i8) -> bool {
            fn is_winning(bricks: &Vec<Brick>, x: i8, y: i8, direction: i8, goal: i8, count: i8) -> bool {
                let has_neighbor = |x, y| -> bool {
                    return bricks
                        .into_iter()
                        .any(|brick| brick.x == x + 1 && brick.y == y + 1 * direction);
                };

                if count == goal - 1 {
                    return true;
                }
                if has_neighbor(x, y) {
                    return is_winning(bricks, x + 1, y + 1 * direction, direction, goal, count + 1);
                }
                return false;
            }

            let mut _count = 0;
            let mut _last_brick = bricks[0];
            for _i in 0..bricks.len() {
                let _p = bricks[_i];
                if is_winning(&bricks, _p.x, _p.y, 1, goal, 0) {
                    // Up
                    return true;
                }
                if is_winning(&bricks, _p.x, _p.y, -1, goal, 0) {
                    // Down
                    return true;
                }
            }
            return false;
        }

        fn check_vertical(bricks: Vec<Brick>, x: i8, goal: i8) -> bool {
            return check_line(
                bricks
                    .into_iter()
                    .filter(|brick| brick.x == x)
                    .map(|brick| brick.y)
                    .collect(),
                goal,
            );
        }

        fn check_horizontal(bricks: Vec<Brick>, y: i8, goal: i8) -> bool {
            return check_line(
                bricks
                    .into_iter()
                    .filter(|brick| brick.y == y)
                    .map(|brick| brick.x)
                    .collect(),
                goal,
            );
        }

        fn check_line(mut values: Vec<i8>, goal: i8) -> bool {
            values.sort();

            let mut _count = 0;
            let mut _last_val = values[0];
            for _i in 0..values.len() {
                let _p = values[_i];
                _count = if _p == _last_val + 1 { _count + 1 } else { 0 };
                if _count == goal - 1 {
                    return true;
                }
                _last_val = _p;
            }
            return false;
        }

        let player_bricks: Vec<Brick> = self
            .bricks
            .clone()
            .into_iter()
            .filter(|brick| brick.player.as_str() == self.player.as_str())
            .collect();
        if check_vertical(player_bricks.clone(), x, self.goal) {
            return true;
        }
        if check_horizontal(player_bricks.clone(), y, self.goal) {
            return true;
        }
        if check_diagonal(player_bricks.clone(), self.goal) {
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
    }

    pub fn is_taken(&self, x: i8, y: i8) -> bool {
        return self.get_brick(x, y) != EMPTY_CHAR;
    }

    pub fn get_brick(&self, x: i8, y: i8) -> &str {
        for brick in self.bricks.iter() {
            if brick.x == x && brick.y == y {
                return brick.player.as_str();
            }
        }
        return EMPTY_CHAR;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_test_already_taken() {
        let mut _board = Board::new(3);
        _board.place(0, 0);
        assert_eq!(_board.get_brick(0, 0), _board.player.as_str());
        assert_eq!(_board.is_taken(0, 0), true);
    }

    #[test]
    fn should_win_horizontal() {
        let mut _board = Board::new(3);
        _board.place(-1, 0);
        _board.place(0, 0);
        _board.place(1, 0);
        _board.place(1, 1);
        assert_eq!(_board.is_winning_move(-1, 0), true);
        assert_eq!(_board.is_winning_move(0, 0), true);
        assert_eq!(_board.is_winning_move(1, 0), true);
        assert_eq!(_board.is_winning_move(1, 1), false);
    }

    #[test]
    fn should_win_vertical() {
        let mut _board = Board::new(3);
        _board.place(0, -1);
        _board.place(0, 0);
        _board.place(0, 1);
        _board.place(1, 1);
        assert_eq!(_board.is_winning_move(0, -1), true);
        assert_eq!(_board.is_winning_move(0, 0), true);
        assert_eq!(_board.is_winning_move(0, 1), true);
        assert_eq!(_board.is_winning_move(1, 1), false);
    }

    #[test]
    fn should_win_diagonal_up() {
        let mut _board = Board::new(3);
        _board.place(-1, -1);
        _board.place(0, 0);
        _board.place(1, 1);
        assert_eq!(_board.is_winning_move(-1, -1), true);
        assert_eq!(_board.is_winning_move(0, 0), true);
        assert_eq!(_board.is_winning_move(1, 1), true);
    }

    #[test]
    fn should_win_diagonal_down() {
        let mut _board = Board::new(3);
        _board.place(-1, 1);
        _board.place(0, 0);
        _board.place(1, -1);
        assert_eq!(_board.is_winning_move(-1, 1), true);
        assert_eq!(_board.is_winning_move(0, 0), true);
        assert_eq!(_board.is_winning_move(1, -1), true);
    }

    #[test]
    fn should_not_win_missing_horizontal() {
        let mut _board = Board::new(3);
        _board.place(-2, 0);
        _board.place(-1, 0);
        _board.place(1, 0);
        _board.place(2, 0);
        assert_eq!(_board.is_winning_move(-2, 0), false);
        assert_eq!(_board.is_winning_move(-1, 0), false);
        assert_eq!(_board.is_winning_move(1, 0), false);
        assert_eq!(_board.is_winning_move(2, 0), false);
    }

    #[test]
    fn should_not_win_missing_diagonal() {
        let mut _board = Board::new(3);
        _board.place(-1, -1);
        _board.place(0, 0);
        _board.place(2, 2);
        _board.place(3, 3);
        assert_eq!(_board.is_winning_move(2, 2), false);
        assert_eq!(_board.is_winning_move(3, 3), false);
    }
}
