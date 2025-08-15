use fourmation_core::*;

fn main() {
    // let get_input = || -> Option<Position> {
    //     let mut buffer = String::new();
    //     std::io::stdin().read_line(&mut buffer).ok()?;
    //     let mut buf_iter = buffer.split_whitespace();
    //     let x: usize = buf_iter.next()?.parse().ok()?;
    //     let y: usize = buf_iter.next()?.parse().ok()?;
    //     Position::from_coordinate(x, y)
    // };

    // let mut game = NextState::Cont(State::from_empty());

    // loop {
    //     match game {
    //         NextState::Cont(ref state) => {
    //             println!("{}", state);
    //             println!("{:?}", state.get_next_position());
    //             if let Some(position) = get_input() {
    //                 if let Some(next_game) = state.fourmation_turn(&Action {
    //                     player: state.get_next_player(),
    //                     position: position,
    //                 }) {
    //                     game = next_game;
    //                 } else {
    //                     println!("Invalid Move");
    //                 }
    //             } else {
    //                 println!("Invalid Input");
    //             }
    //         }
    //         NextState::Done(winner) => {
    //             match winner {
    //                 Some(player) => println!("{}", player),
    //                 None => println!("DRAW"),
    //             };
    //             break;
    //         }
    //     }
    // }
}
