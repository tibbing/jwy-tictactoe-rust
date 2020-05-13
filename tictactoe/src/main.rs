fn main() {
  println!("Starting new game...");

  let points = Vec::new();
  let mut _board: Board = Board { points };

  _board = place(_board, Point {x:0, y:0, player:Player::Player1});
  _board = place(_board, Point {x:0, y:1, player:Player::Player2});
  _board = place(_board, Point {x:1, y:1, player:Player::Player1});
  _board = place(_board, Point {x:1, y:0, player:Player::Player2});
  _board = place(_board, Point {x:0, y:2, player:Player::Player2});

  show(_board);
  println!("Game over!");

}

fn place(mut _board:Board, _point: Point) -> Board {
  _board.points.push(_point);
   return _board
}

fn show(_board:Board){
  for point in _board.points.iter() {
    println!("{0} at {1},{2}", point.player.as_str(), point.x, point.y);
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Player {
  Player1,
  Player2,
}

impl Player {
  pub fn as_str(&self) -> &str {
    match self {
        &Player::Player1 => "X",
        &Player::Player2 => "O",
    }
}
}

#[derive(Copy, Clone, Debug)]
struct Point {
  x: i32,
  y: i32,
  player: Player
}

#[derive(Clone)]
struct Board {
  points: Vec<Point>
}