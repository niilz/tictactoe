mod board;
pub mod player;

use board::Board;
use player::Player;
use std::io::stdin;

const fn options() -> [&'static str; 5] {
    [
        "These are your options:",
        "",
        "q | quit    exits TicTacToe",
        "p | play    starts the game",
        "",
    ]
}

#[derive(Default)]
pub struct Game {
    pub board: Board,
    started: bool,
    stopped: bool,
    player: Option<Player>,
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

        'game: loop {
            if self.stopped {
                break;
            }
            let mut input = String::new();
            if !self.has_started() {
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        let cleaned_input = cleaned(&input);
                        if is_quit(cleaned_input) {
                            break;
                        };
                        if ["p", "play"].contains(&cleaned_input) {
                            self.init();
                            continue;
                        }
                    }
                    Err(e) => eprintln!("Ooops. Err: {}", e),
                }
            }

            loop {
                println!("It is player {:?}'s' turn", self.player.unwrap());
                println!();

                self.board.draw();

                match (self.row, self.col) {
                    (None, None) => {
                        println!("Please enter a row number");
                        println!();
                    }
                    (Some(_), None) => {
                        println!("Please enter a column number");
                        println!();
                    }
                    (Some(row), Some(col)) => {
                        if let Some(player) = self.player {
                            match self.board.set_value(player, (row, col)) {
                                Ok(()) => {
                                    self.swap_player();
                                    self.reset_row_col();
                                    continue;
                                }
                                Err(e) => println!("{}", e),
                            }
                        }
                    }
                    (None, Some(_)) => unreachable!("Col can not be set before Row"),
                }

                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => match cleaned(&input) {
                        val if is_quit(val) => {
                            break 'game;
                        }
                        val => match val.parse::<u32>() {
                            Ok(value) => {
                                if self.row.is_none() {
                                    self.row = Some(value as usize);
                                    continue;
                                } else {
                                    self.col = Some(value as usize);
                                    continue;
                                }
                            }
                            Err(e) => {
                                println!(
                                        "Please enter a valid row-number. The range is: 0 - {}, Err: {}",
                                        self.board.get_height(),
                                        e
                                    );
                                continue;
                            }
                        },
                    },
                    Err(e) => eprintln!("Nope. Err: {}", e),
                }
            }
        }

        println!("Thanks for playing. Come back soon!");
    }

    fn init(&mut self) {
        self.player = Some(Player::ONE);
        self.started = true;
    }

    fn has_started(&self) -> bool {
        self.started
    }

    fn reset_row_col(&mut self) {
        self.row = None;
        self.col = None;
    }

    fn swap_player(&mut self) {
        assert!(self.player.is_some());
        let next_player = if self.player == Some(Player::ONE) {
            Player::TWO
        } else {
            Player::ONE
        };

        self.player = Some(next_player);
    }
}

fn cleaned(input: &str) -> &str {
    input.trim()
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
    use crate::game::cleaned;
    use crate::game::is_quit;
    use crate::game::player::Player;
    use crate::Game;

    #[test]
    fn cleaned_works() {
        let with_white_space = "  user input   ";
        let expected_cleaned = "user input";
        assert_eq!(expected_cleaned, cleaned(with_white_space));
    }

    #[test]
    fn setting_start_works() {
        let mut game = Game::new();
        assert!(!game.has_started());
        game.started = true;
        assert!(game.has_started());
    }

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
        game.player = Some(Player::ONE);
        game.swap_player();
        assert_eq!(Player::TWO, game.player.unwrap());
        game.swap_player();
        assert_eq!(Player::ONE, game.player.unwrap());
    }

    #[test]
    fn is_quit_works() {
        assert!(is_quit("q"));
        assert!(is_quit("quit"));
        assert!(!is_quit("not q"));
    }
}
