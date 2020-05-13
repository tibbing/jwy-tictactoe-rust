use ::std::*;

fn main() {
  println!("Starting new game...");
  let points = Vec::new();
  let mut _board: Board = Board { points };

  let mut count = 0u32;
  let mut player:Player = Player::Player1;
  println!("{} turn:",player.as_str());

  loop {
    count += 1;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_n) => {
          println!("Placing marker at {}...", input);
          let pos: Vec<_> = input.trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Not an Integer!"))
            .collect();

          let _pos_x: i32 = pos[0];
          let _pos_y: i32 = pos[1];
          _board = place(_board, Point {x:_pos_x, y:_pos_y, player});

          if let Player::Player1 = player { player = Player::Player2 } else { player = Player::Player1 }
          println!("{} turn:",player.as_str());
        }
      Err(error) => println!("error: {}", error),
    }

    if count == 3 {
        println!("OK, that's enough");
        break;
    }
  }

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