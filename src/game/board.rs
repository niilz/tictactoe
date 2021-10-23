use crate::game::player::Player;

use std::ops::{Deref, DerefMut};

type Values = [[Option<Player>; 3]; 3];

#[derive(Debug, Default)]
pub struct Board {
    values: Values,
}

impl Deref for Board {
    type Target = Values;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl Board {
    pub fn draw(&self) {
        let print_sep = || println!("-----------");

        for (line_num, line) in self.values.iter().enumerate() {
            println!("{}", gen_line(&line));
            if should_print_seperator(line_num, self.values.len()) {
                print_sep();
            }
        }
    }
}

fn gen_line(values: &[Option<Player>; 3]) -> String {
    values
        .iter()
        .map(|p| match p {
            Some(p) => format!(" {} ", p),
            None => "   ".to_string(),
        })
        .collect::<Vec<String>>()
        .join("|")
}

fn should_print_seperator(line_num: usize, board_height: usize) -> bool {
    line_num < board_height - 1
}

impl Board {
    pub fn set_value(&mut self, player: Player, field: (usize, usize)) -> Result<(), &str> {
        let height = self.get_height() - 1;
        let width = self.get_width() - 1;
        let (x, y) = field;
        if x > height || y > width {
            Err("field must be in range of the grid")
        } else {
            self.values[x][y] = Some(player);
            Ok(())
        }
    }

    pub fn get_height(&self) -> usize {
        self.values.len()
    }

    pub fn get_width(&self) -> usize {
        self.values[0].len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_deref_and_use_inner_values() {
        let board = Board::default();
        let matrix_level_1 = board.len();
        assert_eq!(matrix_level_1, 3);
        let matrix_level_2 = board[0].len();
        assert_eq!(matrix_level_2, 3);
    }

    #[test]
    fn can_deref_mut_and_change_inner_values() {
        let mut board = Board::default();
        board[0][0] = Some(Player::ONE);
        assert_eq!(
            [[Some(Player::ONE), None, None], [None; 3], [None; 3]],
            *board
        );
    }

    #[test]
    fn empty_line_gets_created() {
        let expected_line = "   |   |   ";
        let line = gen_line(&[None; 3]);
        assert_eq!(expected_line, line);
    }

    #[test]
    fn line_with_both_players_gets_created() {
        let expected_line = " X |   | O ";
        let line = gen_line(&[Some(Player::ONE), None, Some(Player::TWO)]);
        assert_eq!(expected_line, line);
    }

    #[test]
    fn can_set_player() {
        let mut board = Board::default();
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::TWO, (2, 2));
        let expected_lines = [
            [Some(Player::ONE), None, None],
            [None; 3],
            [None, None, Some(Player::TWO)],
        ];
        assert_eq!(expected_lines, *board);
    }

    #[test]
    fn setting_fields_outside_the_grid_is_an_error() {
        let mut board = Board::default();
        let err = board.set_value(Player::ONE, (0, 3));
        assert_eq!(Err("field must be in range of the grid"), err);
        let err = board.set_value(Player::ONE, (3, 0));
        assert_eq!(Err("field must be in range of the grid"), err);
    }

    #[test]
    fn can_get_height() {
        let board = Board::default();
        assert_eq!(3, board.get_height());
    }

    #[test]
    fn can_get_width() {
        let board = Board::default();
        assert_eq!(3, board.get_width());
    }
}
