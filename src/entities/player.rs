use super::ninja::Ninja;
use crate::components::player_names::PlayerNames;

#[derive(Copy, Clone)]
pub struct Player {
    pub name: PlayerNames,
    pub ninjas: [Ninja; 4],
}

impl Player {
    pub fn new_player_1() -> Self {
        Player::new(PlayerNames::Player1)
    }

    pub fn new_player_2() -> Self {
        Player::new(PlayerNames::Player2)
    }

    pub fn is_in_play(&self) -> bool {
        self.ninjas[0].is_alive()
            || self.ninjas[1].is_alive()
            || self.ninjas[2].is_alive()
            || self.ninjas[3].is_alive()
    }

    fn new(name: PlayerNames) -> Self {
        Self {
            name,
            ninjas: [
                Ninja::default(),
                Ninja::default(),
                Ninja::default(),
                Ninja::default(),
            ],
        }
    }
}

#[cfg(test)]
mod player_should {
    use super::*;
    use crate::components::{attack::Attack, ninja_names::NinjaNames};

    #[test]
    fn have_a_player_1_model() {
        // Given
        let player = Player::new_player_1();

        // Then
        assert_eq!(PlayerNames::Player1, player.name);
        assert_eq!(4, player.ninjas.len());
    }

    #[test]
    fn have_a_player_2_model() {
        // Given
        let player = Player::new_player_1();

        // Then
        assert_eq!(PlayerNames::Player1, player.name);
        assert_eq!(4, player.ninjas.len());
    }

    #[test]
    fn be_in_play() {
        // Given
        let player = Player::new(PlayerNames::Player1);

        // When
        let is_in_play = player.is_in_play();

        // Then
        assert_eq!(true, is_in_play)
    }

    #[test]
    fn be_out_of_play() {
        // Given
        let ninja = Ninja {
            health: 0,
            name: NinjaNames::FujibayashiNagato,
            attack: Attack::default(),
            id: 0,
        };
        let player = Player {
            name: PlayerNames::Player1,
            ninjas: [ninja, ninja, ninja, ninja],
        };

        // When
        let is_in_play = player.is_in_play();

        // Then
        assert_eq!(false, is_in_play)
    }
}
