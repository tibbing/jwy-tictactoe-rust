use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i8,
    pub y: i8,
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
