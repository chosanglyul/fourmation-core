const MAXX: usize = 7;
const MAXY: usize = 7;

pub enum Player { R, B }

pub struct Position { x: usize, y: usize }

impl Position {
    pub fn new(x: usize, y: usize) -> Option<Self> {
        if x < MAXX && y < MAXY {
            Some(Self {x, y})
        } else {
            None
        }
    }
}

pub struct Move {
    player: Player,
    position: Position,
}

pub struct State {
    board: [[Option<Player>; MAXX]; MAXY],
    last_move: Option<Move>,
}

impl std::ops::Index<Position> for State {
    type Output = Option<Player>;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.board[pos.x][pos.y]
    }
}

pub fn fourmation_turn(state: &State, next_move: &Move) {

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
