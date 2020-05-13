use ::std::*;

fn main() {
    println!("Starting new game...");
    let points = Vec::new();
    let mut _board: Board = Board {
        points,
        player: Player::Player1,
        size: 2,
    };

    let mut count = 0u32;
    _board.nextplayer();

    loop {
        count += 1;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                // println!("Placing marker at {}", input);
                let pos: Vec<_> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().expect("Not an Integer!"))
                    .collect();

                let _pos_x: i32 = pos[0];
                let _pos_y: i32 = pos[1];

                _board.place(_pos_x, _pos_y);
            }
            Err(error) => println!("error: {}", error),
        }
        _board.draw();

        if count == 4 {
            break;
        }

        _board.nextplayer();
    }

    println!("Game over!");
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
        if let Player::Player1 = self.player {
            self.player = Player::Player2
        } else {
            self.player = Player::Player1
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
    player: Player,
}

#[derive(Clone)]
struct Board {
    points: Vec<Point>,
    player: Player,
    size: i32,
}
