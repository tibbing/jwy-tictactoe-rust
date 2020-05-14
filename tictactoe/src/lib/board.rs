use crate::lib::brick::Brick;
use crate::lib::direction::Direction;
use crate::lib::player::Player;
use crate::lib::position::Position;
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

    pub fn place(&mut self, pos: Position) {
        let _brick: Brick = Brick {
            position: pos,
            player: self.player,
        };
        self.bricks.push(_brick);
        self.size = cmp::max(cmp::max(pos.x.abs(), pos.y.abs()), self.size);
    }

    pub fn is_within_board(&self, pos: Position) -> bool {
        return pos.x.abs() <= MAX_SIZE && pos.y.abs() <= MAX_SIZE;
    }

    pub fn is_winning_move(&self, pos: Position) -> bool {
        fn check_diagonal(bricks: Vec<Brick>, goal: i8) -> bool {
            fn is_winning(
                bricks: &Vec<Brick>,
                pos: Position,
                direction: Direction,
                goal: i8,
                count: i8,
            ) -> bool {
                let has_neighbor = |pos: Position| -> bool {
                    return bricks
                        .into_iter()
                        .any(|brick| brick.position == pos.neighbor(direction));
                };

                if count == goal - 1 {
                    return true;
                }
                if has_neighbor(pos) {
                    return is_winning(bricks, pos.neighbor(direction), direction, goal, count + 1);
                }
                return false;
            }

            let mut _count = 0;
            let mut _last_brick = bricks[0];
            for _i in 0..bricks.len() {
                let _brick = bricks[_i];
                if is_winning(&bricks, _brick.position, Direction::UpRight, goal, 0) {
                    return true;
                }
                if is_winning(&bricks, _brick.position, Direction::DownRight, goal, 0) {
                    return true;
                }
            }
            return false;
        }

        fn check_vertical(bricks: Vec<Brick>, from_x: i8, goal: i8) -> bool {
            return check_line(
                bricks
                    .into_iter()
                    .filter(|brick| brick.position.x == from_x)
                    .map(|brick| brick.position.y)
                    .collect(),
                goal,
            );
        }

        fn check_horizontal(bricks: Vec<Brick>, from_y: i8, goal: i8) -> bool {
            return check_line(
                bricks
                    .into_iter()
                    .filter(|brick| brick.position.y == from_y)
                    .map(|brick| brick.position.x)
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
        if check_vertical(player_bricks.clone(), pos.x, self.goal) {
            return true;
        }
        if check_horizontal(player_bricks.clone(), pos.y, self.goal) {
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
        for y in (-self.size..self.size + 1).rev() {
            for x in -self.size..self.size + 1 {
                let pos = Position { x, y };
                print!("{0} ", self.get_brick(pos));
            }
            println!("");
        }
    }

    pub fn is_taken(&self, pos: Position) -> bool {
        return self.get_brick(pos) != EMPTY_CHAR;
    }

    pub fn get_brick(&self, pos: Position) -> &str {
        for brick in self.bricks.iter() {
            if brick.position == pos {
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
        let pos = Position::origo();
        _board.place(pos);

        assert_eq!(_board.get_brick(pos), _board.player.as_str());
        assert_eq!(_board.is_taken(pos), true);
    }

    #[test]
    fn should_test_within_board() {
        let mut _board = Board::new(3);
        assert_eq!(_board.is_within_board(Position::origo()), true);
        assert_eq!(
            _board.is_within_board(Position {
                x: MAX_SIZE,
                y: MAX_SIZE
            }),
            true
        );
        assert_eq!(
            _board.is_within_board(Position {
                x: -MAX_SIZE,
                y: -MAX_SIZE
            }),
            true
        );
        assert_eq!(
            _board.is_within_board(Position {
                x: MAX_SIZE + 1,
                y: 0
            }),
            false
        );
        assert_eq!(
            _board.is_within_board(Position {
                x: 0,
                y: MAX_SIZE + 1
            }),
            false
        );
    }

    #[test]
    fn should_win_horizontal() {
        let mut _board = Board::new(3);
        let pos = Position::origo();
        _board.place(pos);
        _board.place(pos.neighbor_right());
        _board.place(pos.neighbor_up());
        _board.place(pos.neighbor_right().neighbor_right());

        assert_eq!(_board.is_winning_move(pos), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_right()), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_up()), false);
    }

    #[test]
    fn should_win_vertical() {
        let mut _board = Board::new(3);
        let pos = Position::origo();
        _board.place(pos);
        _board.place(pos.neighbor_right());
        _board.place(pos.neighbor_up());
        _board.place(pos.neighbor_up().neighbor_up());

        assert_eq!(_board.is_winning_move(pos), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_up()), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_right()), false);
    }

    #[test]
    fn should_win_diagonal_up() {
        let mut _board = Board::new(3);
        let pos = Position::origo();
        _board.place(pos);
        _board.place(pos.neighbor_up_left());
        _board.place(pos.neighbor_down_right());

        assert_eq!(_board.is_winning_move(pos), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_up_left()), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_down_right()), true);
    }

    #[test]
    fn should_win_diagonal_down() {
        let mut _board = Board::new(3);
        let pos = Position::origo();
        _board.place(pos);
        _board.place(pos.neighbor_up_right());
        _board.place(pos.neighbor_down_left());

        assert_eq!(_board.is_winning_move(pos), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_up_right()), true);
        assert_eq!(_board.is_winning_move(pos.neighbor_down_left()), true);
    }

    #[test]
    fn should_not_win_missing_horizontal() {
        let mut _board = Board::new(3);
        let pos = Position::origo();
        _board.place(pos);
        _board.place(pos.neighbor_right());
        _board.place(pos.neighbor_left().neighbor_left());

        assert_eq!(_board.is_winning_move(pos), false);
        assert_eq!(_board.is_winning_move(pos.neighbor_right()), false);
    }

    #[test]
    fn should_not_win_missing_diagonal() {
        let mut _board = Board::new(3);
        let pos = Position::origo();
        _board.place(pos);
        _board.place(pos.neighbor_down_left());
        _board.place(pos.neighbor_up_right().neighbor_up_right());

        assert_eq!(_board.is_winning_move(pos), false);
    }
}
