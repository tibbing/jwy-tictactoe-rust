use crate::lib::player::Player;
use crate::lib::position::Position;

#[derive(Copy, Clone, Debug)]
pub struct Brick {
    pub position: Position,
    pub player: Player,
}
