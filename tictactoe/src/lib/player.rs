#[derive(Copy, Clone, Debug)]
pub enum Player {
    PlayerX,
    PlayerO,
}

impl Player {
    pub fn as_str(&self) -> &str {
        match self {
            &Player::PlayerX => "X",
            &Player::PlayerO => "O",
        }
    }
}