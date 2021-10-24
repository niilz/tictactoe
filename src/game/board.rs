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
        let print_sep = || println!("      ---+---+---");

        println!("      col col col");
        println!("       1   2   3");
        for (line_num, line) in self.values.iter().enumerate() {
            println!("row {} {}", line_num + 1, gen_line(&line));
            if should_print_seperator(line_num, self.values.len()) {
                print_sep();
            }
        }
    }

    fn get_diags(
        &self,
    ) -> (
        impl Iterator<Item = Option<Player>> + Clone + '_,
        impl Iterator<Item = Option<Player>> + Clone + '_,
    ) {
        let tl_br = self.iter().enumerate().map(|(idx, row)| row[idx]);
        let bl_tr = self.iter().rev().enumerate().map(|(idx, row)| row[idx]);
        (tl_br, bl_tr)
    }

    fn get_rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = Option<Player>> + Clone + '_> + Clone + '_ {
        self.iter().map(|row| row.iter().map(|&item| item))
    }

    fn get_cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = Option<Player>> + Clone + '_> + Clone + '_ {
        (0..3).map(move |row| (0..3).map(move |col| self.values[col][row]))
    }

    pub fn get_winner(&self) -> Option<Player> {
        let diags = self.get_diags();
        if let Some(winner) = check_for_winner(diags.0) {
            return Some(winner);
        }
        if let Some(winner) = check_for_winner(diags.1) {
            return Some(winner);
        }
        for row in self.get_rows() {
            if let Some(winner) = check_for_winner(row) {
                return Some(winner);
            }
        }
        for col in self.get_cols() {
            if let Some(winner) = check_for_winner(col) {
                return Some(winner);
            }
        }
        None
    }
}

fn check_for_winner(mut cells: impl Iterator<Item = Option<Player>> + Clone) -> Option<Player> {
    if cells.clone().all(|p| p == Some(Player::ONE)) {
        Some(Player::ONE)
    } else if cells.all(|p| p == Some(Player::TWO)) {
        Some(Player::TWO)
    } else {
        None
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
            Err("Field must be in range of the grid")
        } else if self.values[x][y].is_some() {
            Err("Field has already been chosen. Please choose another field.")
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
        assert_eq!(Err("Field must be in range of the grid"), err);
        let err = board.set_value(Player::ONE, (3, 0));
        assert_eq!(Err("Field must be in range of the grid"), err);
    }

    #[test]
    fn setting_field_thats_already_set_is_an_error() {
        let mut board = Board::default();
        let _ = board.set_value(Player::ONE, (0, 0));
        let err = board.set_value(Player::ONE, (0, 0));
        assert_eq!(
            Err("Field has already been chosen. Please choose another field."),
            err
        );
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

    #[test]
    fn can_get_diagonals() {
        let mut board = Board::default();
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::ONE, (1, 1));
        let _ = board.set_value(Player::ONE, (2, 2));

        let expected_tl_br = vec![Some(Player::ONE), Some(Player::ONE), Some(Player::ONE)];
        let (tl_br, _) = board.get_diags();
        assert_eq!(expected_tl_br, tl_br.collect::<Vec<_>>());

        let mut board = Board::default();
        let _ = board.set_value(Player::TWO, (0, 2));
        let _ = board.set_value(Player::TWO, (1, 1));
        let _ = board.set_value(Player::TWO, (2, 0));

        let expected_bl_tr = vec![Some(Player::TWO), Some(Player::TWO), Some(Player::TWO)];
        let (_, bl_tr) = board.get_diags();
        assert_eq!(expected_bl_tr, bl_tr.collect::<Vec<_>>());
    }

    #[test]
    fn can_get_rows() {
        let mut board = Board::default();
        // row-0
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::ONE, (0, 1));
        let _ = board.set_value(Player::ONE, (0, 2));

        // row-1
        let _ = board.set_value(Player::TWO, (1, 0));
        let _ = board.set_value(Player::TWO, (1, 1));
        let _ = board.set_value(Player::TWO, (1, 2));

        // row-2
        let _ = board.set_value(Player::ONE, (2, 0));
        let _ = board.set_value(Player::ONE, (2, 1));
        let _ = board.set_value(Player::ONE, (2, 2));

        let expected_rows = vec![
            vec![Some(Player::ONE), Some(Player::ONE), Some(Player::ONE)],
            vec![Some(Player::TWO), Some(Player::TWO), Some(Player::TWO)],
            vec![Some(Player::ONE), Some(Player::ONE), Some(Player::ONE)],
        ];
        let rows: Vec<_> = board
            .get_rows()
            .map(|row| row.collect::<Vec<_>>())
            .collect();
        assert_eq!(expected_rows, rows);
    }

    #[test]
    fn can_get_cols() {
        let mut board = Board::default();
        // col-0
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::ONE, (1, 0));
        let _ = board.set_value(Player::ONE, (2, 0));

        // col-1
        let _ = board.set_value(Player::TWO, (0, 1));
        let _ = board.set_value(Player::TWO, (1, 1));
        let _ = board.set_value(Player::TWO, (2, 1));

        // col-2
        let _ = board.set_value(Player::ONE, (0, 2));
        let _ = board.set_value(Player::ONE, (1, 2));
        let _ = board.set_value(Player::ONE, (2, 2));

        let expected_cols = vec![
            vec![Some(Player::ONE), Some(Player::ONE), Some(Player::ONE)],
            vec![Some(Player::TWO), Some(Player::TWO), Some(Player::TWO)],
            vec![Some(Player::ONE), Some(Player::ONE), Some(Player::ONE)],
        ];
        let cols: Vec<_> = board
            .get_cols()
            .map(|col| col.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(expected_cols, cols);
    }

    #[test]
    fn check_for_winner_finds_winner() {
        let mut board = Board::default();
        // diag_tl_br (ONE is Winner)
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::ONE, (1, 1));
        let _ = board.set_value(Player::ONE, (2, 2));

        let winner = check_for_winner(board.get_diags().0);
        assert_eq!(Some(Player::ONE), winner);

        // diag_bl_tr (TWO is Winner)
        let mut board = Board::default();
        let _ = board.set_value(Player::TWO, (0, 2));
        let _ = board.set_value(Player::TWO, (1, 1));
        let _ = board.set_value(Player::TWO, (2, 0));

        let winner = check_for_winner(board.get_diags().1);
        assert_eq!(Some(Player::TWO), winner);

        // Nobody is winner anymore
        let mut board = Board::default();
        let _ = board.set_value(Player::ONE, (0, 2));
        let _ = board.set_value(Player::TWO, (0, 0));

        let no_winner = check_for_winner(board.get_diags().0);
        assert_eq!(None, no_winner);

        let no_winner = check_for_winner(board.get_diags().1);
        let _ = board.set_value(Player::ONE, (1, 2));
        assert_eq!(None, no_winner);
    }

    #[test]
    fn line_that_contains_none_is_not_a_winner() {
        let line = vec![Some(Player::ONE), Some(Player::ONE), None];
        let no_winner = check_for_winner(line.into_iter());
        assert_eq!(None, no_winner);
    }

    #[test]
    fn winner_in_row_gets_detected() {
        let mut board = Board::default();
        // row-0
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::ONE, (0, 1));
        let _ = board.set_value(Player::ONE, (0, 2));

        // row-1
        let _ = board.set_value(Player::TWO, (1, 0));
        let _ = board.set_value(Player::TWO, (1, 1));
        let _ = board.set_value(Player::TWO, (1, 2));

        // row-2
        let _ = board.set_value(Player::ONE, (2, 0));
        let _ = board.set_value(Player::ONE, (2, 2));

        let mut rows = board.get_rows();

        let winner = check_for_winner(rows.next().unwrap());
        assert_eq!(Some(Player::ONE), winner);

        let winner = check_for_winner(rows.next().unwrap());
        assert_eq!(Some(Player::TWO), winner);

        let no_winner = check_for_winner(rows.next().unwrap());
        assert_eq!(None, no_winner);
    }

    #[test]
    fn winner_in_cols_gets_detected() {
        let mut board = Board::default();
        // col-0
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::ONE, (1, 0));
        let _ = board.set_value(Player::ONE, (2, 0));

        // col-1
        let _ = board.set_value(Player::TWO, (0, 1));
        let _ = board.set_value(Player::TWO, (1, 1));
        let _ = board.set_value(Player::TWO, (2, 1));

        // col-2
        let _ = board.set_value(Player::ONE, (0, 2));
        let _ = board.set_value(Player::ONE, (1, 2));

        let mut columns = board.get_cols();

        let winner = check_for_winner(columns.next().unwrap());
        assert_eq!(Some(Player::ONE), winner);

        let winner = check_for_winner(columns.next().unwrap());
        assert_eq!(Some(Player::TWO), winner);

        let no_winner = check_for_winner(columns.next().unwrap());
        assert_eq!(None, no_winner);
    }

    #[test]
    fn get_winner_returns_winner_if_there_is_one() {
        let mut board = Board::default();

        // One wins in col-0
        let _ = board.set_value(Player::ONE, (0, 0));
        let _ = board.set_value(Player::ONE, (1, 0));
        let _ = board.set_value(Player::ONE, (2, 0));

        let winner = board.get_winner();
        assert_eq!(Some(Player::ONE), winner);

        let mut board = Board::default();

        // Two wins diag top_left-to-bottom_right
        let _ = board.set_value(Player::TWO, (0, 0));
        let _ = board.set_value(Player::TWO, (1, 1));
        let _ = board.set_value(Player::TWO, (2, 2));

        let winner = board.get_winner();
        assert_eq!(Some(Player::TWO), winner);

        // One wins with row-1
        let mut board = Board::default();

        let _ = board.set_value(Player::ONE, (1, 0));
        let _ = board.set_value(Player::ONE, (1, 1));
        let _ = board.set_value(Player::ONE, (1, 2));

        let winner = board.get_winner();
        assert_eq!(Some(Player::ONE), winner);

        // No one wins if no line is filled by one player
        let mut board = Board::default();

        //  two | one | two
        //  ----+-----+----
        //  one | one | two
        //  ----+-----+----
        //  two | two | one
        let _ = board.set_value(Player::TWO, (0, 0));
        let _ = board.set_value(Player::ONE, (0, 1));
        let _ = board.set_value(Player::TWO, (0, 2));

        let _ = board.set_value(Player::ONE, (1, 0));
        let _ = board.set_value(Player::ONE, (1, 1));
        let _ = board.set_value(Player::TWO, (1, 2));

        let _ = board.set_value(Player::TWO, (2, 0));
        let _ = board.set_value(Player::TWO, (2, 1));
        let _ = board.set_value(Player::ONE, (2, 2));

        let no_winner = board.get_winner();
        assert_eq!(None, no_winner);
    }
}
