use crate::components::{attack::Attack, ninja_names::NinjaNames};
use rand::random;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(PartialEq, RandGen, Copy, Clone)]
pub struct Ninja {
    pub id: i64,
    pub name: NinjaNames,
    pub health: u8,
    pub attack: Attack,
}

impl Default for Ninja {
    fn default() -> Self {
        Self {
            id: random(),
            name: random(),
            health: 100,
            attack: Attack::default(),
        }
    }
}

impl Ninja {
    pub fn take_damage(&mut self, attack: Attack) {
        if attack.damage < self.health {
            self.health -= attack.damage;
        } else {
            self.health = 0;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl Display for Ninja {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "| 🥷 {} 🥷 | ❤️  {} ❤️  |", self.name, self.health)
    }
}

#[cfg(test)]
mod ninja_should {
    use super::*;
    use crate::components::attack_names::AttackNames;

    #[test]
    fn have_a_model() {
        // Given
        let ninja = Ninja::default();

        // Then
        assert_ne!(String::default(), ninja.name.to_string());
        assert_eq!(100, ninja.health);
    }

    #[test]
    fn take_damage() {
        // Given
        let attack = Attack::default();
        let mut ninja = Ninja::default();

        // When
        ninja.take_damage(attack);

        // Then
        assert_eq!(75, ninja.health);
    }

    #[test]
    fn take_alot_of_damage() {
        // Given
        let attack = Attack {
            name: AttackNames::FlyingFist,
            damage: 101,
        };
        let mut ninja = Ninja::default();

        // When
        ninja.take_damage(attack);

        // Then
        assert_eq!(0, ninja.health);
    }

    #[test]
    fn is_alive() {
        // Given
        let ninja = Ninja::default();

        // When
        let is_alive = ninja.is_alive();

        // Then
        assert!(is_alive)
    }

    #[test]
    fn is_dead() {
        // Given
        let ninja = Ninja {
            id: 0,
            name: NinjaNames::FujibayashiNagato,
            health: 0,
            attack: Attack::default(),
        };

        // When
        let is_alive = ninja.is_alive();

        // Then
        assert!(!is_alive)
    }
}
