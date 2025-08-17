use std::fmt;
use std::ops::*;

pub type Position = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board(u64);

impl Board {
    pub fn new(value: u64) -> Self {
        assert!(
            value < 0x2000000000000,
            "BitBoard must be smaller than 2^49"
        );

        Board(value)
    }

    pub fn singleton(position: Position) -> Self {
        assert!(position < 49, "Position must be smaller than 49");

        Board(1 << position)
    }

    pub fn get(&self, position: Position) -> bool {
        assert!(position < 49, "Position must be smaller than 49");

        (self.0 >> position) & 1 != 0
    }

    #[inline]
    pub fn east(&self) -> Self {
        Board((self.0 << 1) & 0x1FBF7EFDFBF7E)
    }

    #[inline]
    pub fn north(&self) -> Self {
        Board(self.0 >> 7)
    }

    #[inline]
    pub fn northeast(&self) -> Self {
        Board((self.0 >> 6) & 0x1FBF7EFDFBF7E)
    }

    #[inline]
    pub fn northwest(&self) -> Self {
        Board((self.0 >> 8) & 0xFDFBF7EFDFBF)
    }

    #[inline]
    pub fn south(&self) -> Self {
        Board(self.0 << 7)
    }

    #[inline]
    pub fn southeast(&self) -> Self {
        Board((self.0 << 8) & 0x1FBF7EFDFBF7E)
    }

    #[inline]
    pub fn southwest(&self) -> Self {
        Board((self.0 << 6) & 0xFDFBF7EFDFBF)
    }

    #[inline]
    pub fn west(&self) -> Self {
        Board((self.0 >> 1) & 0xFDFBF7EFDFBF)
    }

    pub fn neighbors(&self) -> Self {
        self.east()
            | self.north()
            | self.northeast()
            | self.northwest()
            | self.south()
            | self.southeast()
            | self.southwest()
            | self.west()
    }
}

impl BitAnd for Board {
    type Output = Board;

    #[inline]
    fn bitand(self, other: Board) -> Board {
        Board(self.0 & other.0)
    }
}

impl BitAnd for &Board {
    type Output = Board;

    #[inline]
    fn bitand(self, other: &Board) -> Board {
        Board(self.0 & other.0)
    }
}

impl BitAnd<&Board> for Board {
    type Output = Board;

    #[inline]
    fn bitand(self, other: &Board) -> Board {
        Board(self.0 & other.0)
    }
}

impl BitAnd<Board> for &Board {
    type Output = Board;

    #[inline]
    fn bitand(self, other: Board) -> Board {
        Board(self.0 & other.0)
    }
}

impl BitOr for Board {
    type Output = Board;

    #[inline]
    fn bitor(self, other: Board) -> Board {
        Board(self.0 | other.0)
    }
}

impl BitOr for &Board {
    type Output = Board;

    #[inline]
    fn bitor(self, other: &Board) -> Board {
        Board(self.0 | other.0)
    }
}

impl BitOr<&Board> for Board {
    type Output = Board;

    #[inline]
    fn bitor(self, other: &Board) -> Board {
        Board(self.0 | other.0)
    }
}

impl BitOr<Board> for &Board {
    type Output = Board;

    #[inline]
    fn bitor(self, other: Board) -> Board {
        Board(self.0 | other.0)
    }
}

impl Not for Board {
    type Output = Board;

    #[inline]
    fn not(self) -> Board {
        Board(!self.0)
    }
}

impl Not for &Board {
    type Output = Board;

    #[inline]
    fn not(self) -> Board {
        Board(!self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    R,
    B,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::R => Player::B,
            Player::B => Player::R,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Action {
    pub player: Player,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub r_board: Board,
    pub b_board: Board,
    pub previous_action: Option<Action>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();

        for i in 0..49 {
            if self.r_board.get(i) {
                s.push_str("R ");
            } else if self.b_board.get(i) {
                s.push_str("B ");
            } else {
                s.push_str("X ");
            }

            if i % 7 == 6 {
                s.push('\n');
            }
        }

        write!(f, "{}", s)
    }
}

impl State {
    pub fn from_empty() -> Self {
        State {
            r_board: Board(0),
            b_board: Board(0),
            previous_action: None,
        }
    }

    pub fn player(&self) -> Player {
        match self.previous_action {
            Some(action) => action.player.opponent(),
            None => Player::R,
        }
    }

    fn empty_board(&self) -> Board {
        !(self.b_board | self.r_board)
    }

    pub fn step(&self, action: &Action) -> Result<(Self, bool), String> {
        if self.player() != action.player {
            return Err(format!(
                "Invalid move: it's {:?}'s turn, but the action was made by {:?}.",
                self.player(),
                action.player
            ));
        }

        if !self.empty_board().get(action.position) {
            return Err("Invalid move: the position is already occupied.".to_string());
        }

        if self.previous_action.is_some_and(|previous_action| {
            let neighbors = Board::singleton(previous_action.position).neighbors();

            if neighbors & !self.empty_board() == neighbors {
                let cp_board = match previous_action.player {
                    Player::R => self.r_board,
                    Player::B => self.b_board,
                };

                Board::singleton(action.position).neighbors() & cp_board == Board(0)
            } else {
                !neighbors.get(action.position)
            }
        }) {
            return Err("Invalid move: the position is not a valid move.".to_string());
        }

        let (r_board, b_board) = match action.player {
            Player::R => (
                self.r_board | Board::singleton(action.position),
                self.b_board,
            ),
            Player::B => (
                self.r_board,
                self.b_board | Board::singleton(action.position),
            ),
        };

        let new_state = State {
            r_board,
            b_board,
            previous_action: Some(*action),
        };

        Ok((new_state, new_state.is_done()))
    }

    fn is_done(&self) -> bool {
        if let Some(previous_action) = self.previous_action {
            let last_position = Board::singleton(previous_action.position);

            let cp_board = match previous_action.player {
                Player::R => self.r_board,
                Player::B => self.b_board,
            };

            let directions = [
                Board::east,
                Board::north,
                Board::northeast,
                Board::northwest,
                Board::south,
                Board::southeast,
                Board::southwest,
                Board::west,
            ];

            for direction in directions {
                let line = (1..4).fold(last_position, |acc, _| acc | direction(&acc));

                if line.0.count_ones() == 4 && (line & cp_board == line) {
                    return true;
                }
            }

            if (self.empty_board() & last_position.neighbors()) == Board(0) {
                return (self.empty_board() & cp_board.neighbors()) == Board(0);
            }
        }

        false
    }
}
