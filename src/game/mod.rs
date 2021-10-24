mod board;
pub mod player;

use board::{Board, BoardState};
use player::Player;
use std::io::stdin;

const fn options() -> [&'static str; 4] {
    [
        "These are your options:",
        "",
        "q | quit    exits TicTacToe",
        "",
    ]
}

#[derive(Default)]
pub struct Game {
    pub board: Board,
    player: Player,
    row: Option<usize>,
    col: Option<usize>,
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn play(&mut self) {
        println!("Welcome to TicTacToe.");
        println!();

        print_lines(options());

        loop {
            match self.board.get_board_state() {
                BoardState::WON(winner) => {
                    println!();
                    println!("Congratulations! Player {:?} won!", winner);
                    self.board.draw();
                    println!();
                    break;
                }
                BoardState::DRAW => {
                    println!("Game Over. Nobody won!");
                    break;
                }
                BoardState::ONGOING => { /* just keep going */ }
            }

            println!();
            println!("It is player {:?}'s' turn", self.player);
            println!();

            self.board.draw();

            match (self.row, self.col) {
                (None, None) => {
                    println!();
                    println!("Please enter a row number");
                    println!();
                }
                (Some(_), None) => {
                    println!();
                    println!("Please enter a column number");
                    println!();
                }
                (Some(row), Some(col)) => match self.board.set_value(self.player, (row, col)) {
                    Ok(()) => {
                        self.swap_player();
                        self.reset_row_col();
                        continue;
                    }
                    Err(e) => {
                        println!();
                        println!("{}", e);
                        self.reset_row_col();
                    }
                },
                (None, Some(_)) => unreachable!("Col can not be set before Row"),
            }

            let mut input = String::new();
            let row_col_input = get_input(&mut input);
            match row_col_input {
                val if is_quit(val) => {
                    break;
                }
                val => match val.parse::<u32>() {
                    Ok(value) if self.row.is_none() && value > 0 => {
                        self.row = Some((value - 1) as usize);
                        continue;
                    }
                    Ok(value) if value > 0 => {
                        self.col = Some((value - 1) as usize);
                        continue;
                    }
                    Ok(_) => {
                        println!(
                            "Please enter a valid number. The range is: 1 - {}",
                            self.board.get_height(),
                        );
                    }
                    Err(_) => {
                        println!(
                            "Please enter a digit between 1 and {}",
                            self.board.get_height(),
                        );
                        continue;
                    }
                },
            }
        }

        println!("Thanks for playing. Come back soon!");
    }

    fn reset_row_col(&mut self) {
        self.row = None;
        self.col = None;
    }

    fn swap_player(&mut self) {
        self.player = if self.player == Player::ONE {
            Player::TWO
        } else {
            Player::ONE
        }
    }
}

fn get_input<'a>(mut input: &'a mut String) -> &'a str {
    match stdin().read_line(&mut input) {
        Ok(_) => input.trim(),
        Err(e) => panic!("Ooops. Couldn't read from stdin. Err: {}", e),
    }
}

fn is_quit(command: &str) -> bool {
    ["q", "quit"].contains(&command)
}

fn print_lines<I>(lines: I)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: std::fmt::Display,
{
    for line in lines {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::is_quit;
    use crate::game::player::Player;
    use crate::Game;

    #[test]
    fn resetting_row_col_works() {
        let mut game = Game::default();
        game.row = Some(1);
        game.col = Some(3);
        game.reset_row_col();
        assert_eq!(None, game.row);
        assert_eq!(None, game.col);
    }

    #[test]
    fn swapping_player_works() {
        let mut game = Game::default();
        game.player = Player::ONE;
        game.swap_player();
        assert_eq!(Player::TWO, game.player);
        game.swap_player();
        assert_eq!(Player::ONE, game.player);
    }

    #[test]
    fn is_quit_works() {
        assert!(is_quit("q"));
        assert!(is_quit("quit"));
        assert!(!is_quit("not q"));
    }
}
