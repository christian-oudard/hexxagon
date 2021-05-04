use std::collections::HashMap;

#[derive(Debug)]
enum Piece {
    Empty,
    Black,
    White,
}
type Pos = (i32, i32);
type Dir = (i32, i32);

type PositionMap = HashMap<Pos, Piece>;

struct Board {
    positions: PositionMap,
}

// Double-width horizontal layout. (https://www.redblobgames.com/grids/hexagons/)
// 0 is the piece position, 1s are one step away, and 2s are one leap away.
//   2 2 2
//  2 1 1 2
// 2 1 0 1 2
//  2 1 1 2
//   2 2 2

const STEP_DIRECTIONS: &'static [(i32, i32)] = &[
    (2, 0),   // E
    (1, 1),   // NE
    (-1, 1),  // NW
    (-2, 0),  // W
    (-1, -1), // SW
    (1, -1),  // SE
];
const LEAP_DIRECTIONS: &'static [(i32, i32)] = &[
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

        let min_x = positions
            .keys()
            .map(|(x, _y)| x)
            .min()
            .expect("empty board");
        let min_y = positions
            .keys()
            .map(|(_x, y)| y)
            .min()
            .expect("empty board");
        assert_eq!(*min_x, 0);
        assert_eq!(*min_y, 0);

        Ok(Board { positions })
    }

    fn at(&self, pos: &Pos) -> Option<&Piece> {
        self.positions.get(pos)
    }

    fn _neighbors(&self, pos: &Pos, directions: &[Dir]) -> Vec<Pos> {
        match self.at(pos) {
            None => vec![],
            Some(_) => {
                let mut result = Vec::new();
                for dir in directions {
                    let n_pos = offset(*pos, *dir);
                    match self.at(&n_pos) {
                        Some(_) => result.push(n_pos),
                        None => continue,
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
}

fn offset((x, y): Pos, (dx, dy): Dir) -> Pos {
    (x + dx, y + dy)
}

fn main() {
    let board = Board::load(
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

    println!("{:?}", board.at(&(6, 2)));
    println!("{:?}", board.step_neighbors(&(6, 2)));
    println!("{:?}", board.leap_neighbors(&(6, 2)));
}
