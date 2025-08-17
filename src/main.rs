use fourmation_core::*;

fn main() {
    let get_input = || -> Option<Position> {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok()?;
        let mut buf_iter = buffer.split_whitespace();
        let x: u64 = buf_iter.next()?.parse().ok()?;
        let y: u64 = buf_iter.next()?.parse().ok()?;
        if x >= 7 || y >= 7 {
            None
        } else {
            Some(Position::new(x * 7 + y))
        }
    };

    let mut state = State {
        r_board: Board::new(0),
        b_board: Board::new(0),
        previous_action: None,
    };

    loop {
        if let Some(position) = get_input() {
            match state.step(&Action {
                player: state.player(),
                position,
            }) {
                Ok((new_state, is_done)) => {
                    state = new_state;

                    println!("{}", state);

                    if is_done {
                        break;
                    }
                }
                Err(err_msg) => {
                    println!("{}", err_msg);
                }
            }
        } else {
            println!("Invalid Input");
        }
    }
}
