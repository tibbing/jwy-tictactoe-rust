use ::std::*;

fn main() {
    println!("Starting new game...");
    let points = Vec::new();
    let mut _board: Board = Board {
        points,
        player: Player::Player1,
    };

    let mut count = 0u32;
    _board = nextplayer(_board);

    loop {
        count += 1;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                println!("Placing marker at {}", input);
                let pos: Vec<_> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().expect("Not an Integer!"))
                    .collect();

                let _pos_x: i32 = pos[0];
                let _pos_y: i32 = pos[1];

                _board = place(_board, _pos_x, _pos_y);
                _board = nextplayer(_board);
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

fn place(mut _board: Board, x: i32, y: i32) -> Board {
    let _point: Point = Point {
        x: x,
        y: y,
        player: _board.player,
    };
    _board.points.push(_point);
    return _board;
}

fn nextplayer(mut _board: Board) -> Board {
    if let Player::Player1 = _board.player {
        _board.player = Player::Player2
    } else {
        _board.player = Player::Player1
    }
    println!("{} turn:", _board.player.as_str());
    return _board;
}

fn show(_board: Board) {
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
    player: Player,
}

#[derive(Clone)]
struct Board {
    points: Vec<Point>,
    player: Player,
}
