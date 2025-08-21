use std::fmt;
use std::ops::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position(u64);

impl Position {
    pub fn new(value: u64) -> Self {
        assert!(value < 49, "Position must be smaller than 49");

        Position(value)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0 / 7, self.0 % 7)
    }
}

impl Shl<Position> for u64 {
    type Output = u64;

    fn shl(self, rhs: Position) -> Self::Output {
        self << rhs.0
    }
}

impl Shr<Position> for u64 {
    type Output = u64;

    fn shr(self, rhs: Position) -> Self::Output {
        self >> rhs.0
    }
}

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

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn singleton(position: Position) -> Self {
        Board(1 << position)
    }

    pub fn get(&self, position: Position) -> bool {
        (self.0 >> position) & 1 != 0
    }

    #[inline]
    pub fn east(&self) -> Self {
        Board((self.0 << 1) & 0x1FBF7EFDFBF7E)
    }

    #[inline]
    pub fn north(&self) -> Self {
        Board((self.0 >> 7) & 0x1FFFFFFFFFFFF)
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
        Board((self.0 << 7) & 0x1FFFFFFFFFFFF)
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
        Board(self.0 ^ 0x1FFFFFFFFFFFF)
    }
}

impl Not for &Board {
    type Output = Board;

    #[inline]
    fn not(self) -> Board {
        Board(self.0 ^ 0x1FFFFFFFFFFFF)
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
            let p = Position::new(i);

            if self.r_board.get(p) {
                s.push_str("R ");
            } else if self.b_board.get(p) {
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

    pub fn empty_board(&self) -> Board {
        !(self.b_board | self.r_board)
    }

    pub fn player_board(&self, player: Player) -> Board {
        match player {
            Player::R => self.r_board,
            Player::B => self.b_board,
        }
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
                Board::singleton(action.position).neighbors()
                    & self.player_board(previous_action.player)
                    == Board(0)
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

    fn next_action_board(&self) -> Board {
        if let Some(previous_action) = self.previous_action {
            let board_possible =
                self.empty_board() & Board::singleton(previous_action.position).neighbors();

            if board_possible == Board(0) {
                self.empty_board() & self.player_board(previous_action.player).neighbors()
            } else {
                board_possible
            }
        } else {
            !Board(0)
        }
    }

    pub fn next_action(&self) -> Vec<Position> {
        let mut bits = self.next_action_board().to_u64();
        let mut result = Vec::with_capacity(bits.count_ones() as usize);

        while bits != 0 {
            result.push(Position(bits.trailing_zeros() as u64));
            bits &= bits - 1;
        }

        result
    }

    fn is_done(&self) -> bool {
        if let Some(previous_action) = self.previous_action {
            [
                Board::east,
                Board::north,
                Board::northeast,
                Board::northwest,
                Board::south,
                Board::southeast,
                Board::southwest,
                Board::west,
            ]
            .into_iter()
            .any(|direction| {
                std::iter::successors(Some(self.player_board(previous_action.player)), |b| {
                    Some(direction(b))
                })
                .take(4)
                .fold(!Board(0), |acc, board| acc & board)
                    != Board(0)
            }) || self.next_action_board() == Board(0)
        } else {
            false
        }
    }
}
