use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    ONE,
    TWO,
}

impl Default for Player {
    fn default() -> Self {
        Player::ONE
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ONE => write!(f, "X"),
            Self::TWO => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_represent_players() {
        let one = format!("{}", Player::ONE);
        let two = format!("{}", Player::TWO);
        assert_eq!(("X".to_string(), "O".to_string()), (one, two));
    }
}
