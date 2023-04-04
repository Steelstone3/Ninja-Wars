use super::attack_names::AttackNames;
use rand::random;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(PartialEq, RandGen, Copy, Clone)]
pub struct Attack {
    pub name: AttackNames,
    pub damage: u8,
}

impl Default for Attack {
    fn default() -> Self {
        Self {
            name: random(),
            damage: 25,
        }
    }
}

impl Display for Attack {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {}", self.name, self.damage)
    }
}

#[cfg(test)]
mod attack_should {
    use super::*;

    #[test]
    fn have_a_model() {
        // Given
        let attack = Attack::default();

        // Then
        assert_eq!(25, attack.damage);
        assert_ne!(String::default(), attack.name.to_string());
    }
}
