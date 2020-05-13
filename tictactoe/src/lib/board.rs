use ::std::*;
use crate::lib::player::Player;
use crate::lib::point::Point;

#[derive(Clone)]
pub struct Board {
    pub points: Vec<Point>,
    pub player: Player,
    pub size: i32,
}

impl Board {
  pub fn place(&mut self, x: i32, y: i32) {
      let _point: Point = Point {
          x: x,
          y: y,
          player: self.player,
      };
      self.points.push(_point);
      self.size = cmp::max(cmp::max(x.abs(), y.abs()), self.size)
  }

  pub fn nextplayer(&mut self) {
      if let Player::PlayerX = self.player {
          self.player = Player::PlayerO
      } else {
          self.player = Player::PlayerX
      }
      println!("{} turn:", self.player.as_str());
  }

  pub fn draw(&self) {
      println!("");
      for _y in (-self.size..self.size + 1).rev() {
          for _x in -self.size..self.size + 1 {
              print!("{0} ",self.get_point(_x,_y));
          }
          println!("");
      }
      // for point in self.points.iter() {
      //     println!("{0} at {1},{2}", point.player.as_str(), point.x, point.y);
      // }
  }

  pub fn get_point(&self, x: i32, y: i32) -> &str {
    for point in self.points.iter() {
      if point.x == x && point.y == y {
        return point.player.as_str();
      }
    }
    return "·";
  }
}