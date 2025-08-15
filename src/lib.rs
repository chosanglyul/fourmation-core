pub const MAXX: usize = 7;
pub const MAXY: usize = 7;
pub const FOUR: usize = 4;

use std::fmt;
use std::ops::*;

pub type Position = u64;

#[derive(Debug, Clone, Copy, PartialEq)]
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

        self.0 & (1 << position) != 0
    }

    pub fn set(&self, position: Position) -> Self {
        assert!(position < 49, "Position must be smaller than 49");

        Board(self.0 | (1 << position))
    }

    pub fn east(&self) -> Self {
        Board((self.0 << 1) & 0x1FBF7EFDFBF7E)
    }

    pub fn north(&self) -> Self {
        Board(self.0 >> 7)
    }

    pub fn northeast(&self) -> Self {
        Board((self.0 >> 6) & 0x1FBF7EFDFBF7E)
    }

    pub fn northwest(&self) -> Self {
        Board((self.0 >> 8) & 0xFDFBF7EFDFBF)
    }

    pub fn south(&self) -> Self {
        Board(self.0 << 7)
    }

    pub fn southeast(&self) -> Self {
        Board((self.0 << 8) & 0x1FBF7EFDFBF7E)
    }

    pub fn southwest(&self) -> Self {
        Board((self.0 << 6) & 0xFDFBF7EFDFBF)
    }

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

impl BitXor for Board {
    type Output = Board;

    #[inline]
    fn bitxor(self, other: Board) -> Board {
        Board(self.0 ^ other.0)
    }
}

impl BitXor for &Board {
    type Output = Board;

    #[inline]
    fn bitxor(self, other: &Board) -> Board {
        Board(self.0 ^ other.0)
    }
}

impl BitXor<&Board> for Board {
    type Output = Board;

    #[inline]
    fn bitxor(self, other: &Board) -> Board {
        Board(self.0 ^ other.0)
    }
}

impl BitXor<Board> for &Board {
    type Output = Board;

    #[inline]
    fn bitxor(self, other: Board) -> Board {
        Board(self.0 ^ other.0)
    }
}

impl BitAndAssign for Board {
    #[inline]
    fn bitand_assign(&mut self, other: Board) {
        self.0 &= other.0;
    }
}

impl BitAndAssign<&Board> for Board {
    #[inline]
    fn bitand_assign(&mut self, other: &Board) {
        self.0 &= other.0;
    }
}

impl BitOrAssign for Board {
    #[inline]
    fn bitor_assign(&mut self, other: Board) {
        self.0 |= other.0;
    }
}

impl BitOrAssign<&Board> for Board {
    #[inline]
    fn bitor_assign(&mut self, other: &Board) {
        self.0 |= other.0;
    }
}

impl BitXorAssign for Board {
    #[inline]
    fn bitxor_assign(&mut self, other: Board) {
        self.0 ^= other.0;
    }
}

impl BitXorAssign<&Board> for Board {
    #[inline]
    fn bitxor_assign(&mut self, other: &Board) {
        self.0 ^= other.0;
    }
}

impl Mul for Board {
    type Output = Board;

    #[inline]
    fn mul(self, other: Board) -> Board {
        Board(self.0.wrapping_mul(other.0))
    }
}

impl Mul for &Board {
    type Output = Board;

    #[inline]
    fn mul(self, other: &Board) -> Board {
        Board(self.0.wrapping_mul(other.0))
    }
}

impl Mul<&Board> for Board {
    type Output = Board;

    #[inline]
    fn mul(self, other: &Board) -> Board {
        Board(self.0.wrapping_mul(other.0))
    }
}

impl Mul<Board> for &Board {
    type Output = Board;

    #[inline]
    fn mul(self, other: Board) -> Board {
        Board(self.0.wrapping_mul(other.0))
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();

        for i in 0..49 {
            s.push_str(if self.0 & (1 << i) == 0 { "X " } else { "O " });

            if i % 7 == 6 {
                s.push('\n');
            }
        }

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    R,
    B,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::R => write!(f, "R"),
            Player::B => write!(f, "B"),
        }
    }
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::R => Player::B,
            Player::B => Player::R,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub player: Player,
    pub position: Position,
}

#[derive(Debug, Clone, Copy)]
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

    fn is_empty(&self, position: Position) -> bool {
        !(self.r_board.get(position) || self.b_board.get(position))
    }

    // TODO: Change Option to Result
    pub fn step(&self, action: &Action) -> Option<(Self, bool)> {
        if self.player() != action.player || !self.is_empty(action.position) {
            return None;
        }

        if !self.previous_action.is_none_or(|previous_action| {
            Board::singleton(previous_action.position)
                .neighbors()
                .get(action.position)
        }) {
            return None;
        }

        let (r_board, b_board) = match action.player {
            Player::R => (self.r_board.set(action.position), self.b_board),
            Player::B => (self.r_board, self.b_board.set(action.position)),
        };

        let new_state = State {
            r_board,
            b_board,
            previous_action: Some(*action),
        };

        Some((new_state, new_state.is_done()))
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
                let line = (0..3).fold(last_position, |acc, _| acc | direction(&acc));

                if line & cp_board == line {
                    return true;
                }
            }

            let neighbors = last_position.neighbors();

            if ((neighbors ^ self.r_board ^ self.b_board) & neighbors) == Board(0) {
                let op_board = match previous_action.player {
                    Player::R => self.b_board,
                    Player::B => self.r_board,
                };

                let neighbors = op_board.neighbors();

                return ((neighbors ^ self.r_board ^ self.b_board) & neighbors) == Board(0);
            }
        }

        false
    }
}
