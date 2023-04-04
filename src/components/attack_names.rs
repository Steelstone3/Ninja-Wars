use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Copy, Clone)]
pub enum AttackNames {
    FlyingFist,
    KatanaSlash,
    PowerKick,
}

impl Display for AttackNames {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttackNames::FlyingFist => {
                write!(formatter, "Flying Fist")
            }
            AttackNames::KatanaSlash => {
                write!(formatter, "Katana Slash")
            }
            AttackNames::PowerKick => {
                write!(formatter, "Power Kick")
            }
        }
    }
}
