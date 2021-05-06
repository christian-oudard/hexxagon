use std::collections::{hash_map::Entry, HashMap};
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
enum Piece {
    Empty,
    Black,
    White,
}

impl Piece {
    fn opposite(&self) -> Piece {
        match self {
            Piece::Empty => Piece::Empty,
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}

type Pos = (i32, i32);
type Dir = (i32, i32);

// Double-width horizontal layout. (https://www.redblobgames.com/grids/hexagons/)
// 0 is the piece position, 1s are one step away, and 2s are one leap away.
//   2 2 2
//  2 1 1 2
// 2 1 0 1 2
//  2 1 1 2
//   2 2 2

const STEP_DIRECTIONS: &'static [Dir] = &[
    (2, 0),   // E
    (1, 1),   // NE
    (-1, 1),  // NW
    (-2, 0),  // W
    (-1, -1), // SW
    (1, -1),  // SE
];
const LEAP_DIRECTIONS: &'static [Dir] = &[
    (4, 0),   // E
    (3, 1),   // ENE
    (2, 2),   // NE
    (0, 2),   // N
    (-2, 2),  // NW
    (-3, 1),  // WNW
    (-4, 0),  // W
    (-3, -1), // WSW
    (-2, -2), // SW
    (0, -2),  // S
    (2, -2),  // SE
    (3, -1),  // ESE
];

enum Move {
    Step(Pos),
    Leap(Pos, Pos),
}

type PositionMap = HashMap<Pos, Piece>;
struct Board {
    positions: PositionMap,
    turn: Piece,
}

impl Board {
    fn load(input: &str) -> Result<Board, String> {
        let dedented = textwrap::dedent(input);
        let lines = dedented
            .lines()
            .map(|l| l.trim_end())
            .filter(|l| !l.is_empty());

        let mut positions: PositionMap = HashMap::new();
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                // let value: Option<Hex> = match c {
                let value = match c {
                    '-' => Some(Piece::Empty),
                    'X' => Some(Piece::Black),
                    'O' => Some(Piece::White),
                    _ => None,
                };
                match value {
                    Some(piece) => {
                        let pos = (x as i32, y as i32);
                        positions.insert(pos, piece);
                    }
                    None => continue,
                };
            }
        }
        if positions.len() == 0 {
            return Err("No pieces on board.".into());
        }

        let board = Board {
            positions,
            turn: Piece::Black,
        };
        assert_eq!(board.min_x(), 0);
        assert_eq!(board.min_y(), 0);
        Ok(board)
    }

    fn at(&self, pos: &Pos) -> Option<&Piece> {
        self.positions.get(pos)
    }

    fn set(&mut self, pos: &Pos, piece: Piece) {
        if let Entry::Occupied(mut entry) = self.positions.entry(*pos) {
            entry.insert(piece);
        }
    }

    fn min_x(&self) -> i32 {
        *self
            .positions
            .keys()
            .map(|(x, _y)| x)
            .min()
            .expect("empty board")
    }

    fn max_x(&self) -> i32 {
        *self
            .positions
            .keys()
            .map(|(x, _y)| x)
            .max()
            .expect("empty board")
    }

    fn min_y(&self) -> i32 {
        *self
            .positions
            .keys()
            .map(|(_x, y)| y)
            .min()
            .expect("empty board")
    }

    fn max_y(&self) -> i32 {
        *self
            .positions
            .keys()
            .map(|(_x, y)| y)
            .max()
            .expect("empty board")
    }

    fn _neighbors(&self, pos: &Pos, directions: &[Dir]) -> Vec<Pos> {
        match self.at(pos) {
            None => vec![], // A position off the board has no neighbors.
            Some(_) => {
                let mut result = Vec::new();
                for dir in directions {
                    let n_pos = offset(*pos, *dir);
                    match self.at(&n_pos) {
                        Some(_) => result.push(n_pos),
                        None => continue, // Only return neighbors which are actual positions.
                    }
                }
                result
            }
        }
    }

    fn step_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        self._neighbors(pos, STEP_DIRECTIONS)
    }

    fn leap_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        self._neighbors(pos, LEAP_DIRECTIONS)
    }

    fn do_move(&mut self, mv: &Move) {
        match mv {
            Move::Step(pos) => {
                self.set(pos, self.turn.clone());
                self.flip_neighbors(pos);
            }
            Move::Leap(start, end) => {
                self.set(start, Piece::Empty);
                self.set(end, self.turn.clone());
                self.flip_neighbors(end);
            }
        }
        self.turn = self.turn.opposite();
    }

    fn flip_neighbors(&mut self, pos: &Pos) {
        for n in self.step_neighbors(pos) {
            if let Some(p) = self.at(&n) {
                if *p == self.turn.opposite() {
                    self.set(&n, self.turn.clone());
                }
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min_x, max_x, min_y, max_y) = (self.min_x(), self.max_x(), self.min_y(), self.max_y());
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = match self.at(&(x, y)) {
                    None => " ",
                    Some(Piece::Empty) => "-",
                    Some(Piece::Black) => "X",
                    Some(Piece::White) => "O",
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn offset((x, y): Pos, (dx, dy): Dir) -> Pos {
    (x + dx, y + dy)
}

fn main() {
    let mut board = Board::load(
        "
            X - - - O
           - - - - - -
          - - - - - - -
         - - - -   - - -
        O - -   - - - - X
         - - - -   - - -
          - - - - - - -
           - - - - - -
            X - - - O
        ",
    )
    .expect("board error");

    println!("{}", board.to_string());
    board.do_move(&Move::Step((6, 0)));
    println!("{}", board.to_string());
    board.do_move(&Move::Leap((12, 0), (8, 0)));
    println!("{}", board.to_string());
    board.do_move(&Move::Leap((4, 0), (7, 1)));
    println!("{}", board.to_string());
}
