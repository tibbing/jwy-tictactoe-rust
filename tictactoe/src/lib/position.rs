use regex::Regex;
use std::str::FromStr;

use crate::lib::direction::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Position {
    #[allow(dead_code)]
    pub fn origo() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn neighbor(&self, direction: Direction) -> Position {
        return match direction {
            Direction::UpRight => self.neighbor_up_right(),
            Direction::DownRight => self.neighbor_down_right(),
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_left(&self) -> Position {
        return Position {
            x: self.x - 1,
            y: self.y,
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_right(&self) -> Position {
        return Position {
            x: self.x + 1,
            y: self.y,
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_up(&self) -> Position {
        return Position {
            x: self.x,
            y: self.y + 1,
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_down(&self) -> Position {
        return Position {
            x: self.x,
            y: self.y - 1,
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_up_left(&self) -> Position {
        return Position {
            x: self.x - 1,
            y: self.y + 1,
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_up_right(&self) -> Position {
        return Position {
            x: self.x + 1,
            y: self.y + 1,
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_down_left(&self) -> Position {
        return Position {
            x: self.x - 1,
            y: self.y - 1,
        };
    }

    #[allow(dead_code)]
    pub fn neighbor_down_right(&self) -> Position {
        return Position {
            x: self.x + 1,
            y: self.y - 1,
        };
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(r"^(-?[0-9]+) (-?[0-9]+)$")
            .unwrap()
            .captures(s)
            .and_then(|cap| {
                let x = cap.get(1).and_then(|m| m.as_str().parse().ok());
                let y = cap.get(2).and_then(|m| m.as_str().parse().ok());

                match (x, y) {
                    (Some(x), Some(y)) => Some((x, y)),
                    _ => None,
                }
            })
            .map(|(x, y)| Position { x, y })
            .ok_or_else(|| "Position cant be parsed".to_string())
    }
}
