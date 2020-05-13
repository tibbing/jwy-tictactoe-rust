use crate::lib::player::Player;

#[derive(Copy, Clone, Debug)]
pub struct Brick {
    pub x: i32,
    pub y: i32,
    pub player: Player,
}
