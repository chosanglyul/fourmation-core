const MAXX: usize = 7;
const MAXY: usize = 7;
const FOUR: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    R,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Translation {
    x: isize,
    y: isize,
}

impl Translation {
    pub fn from_coordinate(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn scalar_multiplication(&self, c: isize) -> Self {
        Self {
            x: self.x * c,
            y: self.y * c,
        }
    }

    pub fn from_difference(a: Position, b: Position) -> Self {
        Self {
            x: a.x as isize - b.x as isize,
            y: a.y as isize - b.y as isize,
        }
    }

    pub fn is_unit(&self) -> bool {
        if self.x == 0 && self.y == 0 {
            false
        } else if self.x.unsigned_abs() <= 1 && self.y.unsigned_abs() <= 1 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn from_coordinate(x: usize, y: usize) -> Option<Self> {
        if x < MAXX && y < MAXY {
            Some(Self { x, y })
        } else {
            None
        }
    }

    pub fn add_translation(&self, translation: Translation) -> Option<Self> {
        Self::from_coordinate(
            self.x.checked_add_signed(translation.x)?,
            self.y.checked_add_signed(translation.y)?,
        )
    }

    pub fn all_neighbor(&self) -> Vec<Self> {
        [
            (0, 1),
            (1, 0),
            (1, 1),
            (1, -1),
            (0, -1),
            (-1, 0),
            (-1, -1),
            (-1, 1),
        ]
        .iter()
        .map(|&(x, y)| Translation::from_coordinate(x, y))
        .map(|translation| self.add_translation(translation))
        .flatten()
        .collect()
    }

    pub fn to_usize(&self) -> usize {
        self.x * MAXY + self.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Action {
    player: Player,
    position: Position,
}

pub struct State {
    board: Vec<Option<Player>>,
    last_move: Option<Action>,
}

impl std::ops::Index<Position> for State {
    type Output = Option<Player>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.board[index.to_usize()]
    }
}

impl State {
    pub fn from_empty() -> Self {
        State {
            board: (0..MAXX * MAXY).into_iter().map(|_| None).collect(),
            last_move: None,
        }
    }

    pub fn apply_action(&self, next_move: &Action) -> Option<Self> {
        if self[next_move.position].is_some() {
            None
        } else if self.last_move.is_some_and(|last_move| {
            if last_move.player == next_move.player {
                true
            } else if Translation::from_difference(last_move.position, next_move.position).is_unit()
            {
                false
            } else if last_move
                .position
                .all_neighbor()
                .iter()
                .any(|&pos| self[pos].is_none())
            {
                true
            } else {
                false
            }
        }) {
            None
        } else {
            Some(State {
                board: self
                    .board
                    .iter()
                    .enumerate()
                    .map(|(idx, &player)| {
                        if idx == next_move.position.to_usize() {
                            Some(next_move.player)
                        } else {
                            player
                        }
                    })
                    .collect(),
                last_move: Some(*next_move),
            })
        }
    }

    pub fn fourmation_turn(&self, next_move: &Action) -> Option<NextState> {
        let new_state = self.apply_action(next_move)?;
        let last_position = new_state.last_move.unwrap().position;

        if [(0, 1), (1, 0), (1, 1), (1, -1)]
            .iter()
            .map(|&(x, y)| Translation::from_coordinate(x, y))
            .any(|translation| {
                (-3..=3)
                    .into_iter()
                    .map(|c| translation.scalar_multiplication(c))
                    .map(|dir| last_position.add_translation(dir))
                    .flatten()
                    .map(|pos| new_state[pos])
                    .collect::<Vec<_>>()
                    .windows(FOUR)
                    .any(|arr| arr.iter().all(|&player| player == Some(next_move.player)))
            })
        {
            Some(NextState::Done(Some(next_move.player)))
        } else {
            // TODO fast check draw
            // naive: try all apply_action and check all none
            Some(NextState::Cont(new_state))
        }
    }
}

pub enum NextState {
    Done(Option<Player>),
    Cont(State),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
