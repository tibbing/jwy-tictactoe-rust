use crate::lib::player::Player;

#[derive(Copy, Clone, Debug)]
pub struct Brick {
    pub x: i8,
    pub y: i8,
    pub player: Player,
}
