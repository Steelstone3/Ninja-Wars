use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Copy, Clone)]
pub enum PlayerNames {
    Player1,
    Player2,
}

impl Display for PlayerNames {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerNames::Player1 => {
                write!(formatter, "Player 1")
            }
            PlayerNames::Player2 => {
                write!(formatter, "Player 2")
            }
        }
    }
}
