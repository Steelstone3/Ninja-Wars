use crate::{
    entities::{ninja::Ninja, player::Player},
    presenters::{ninja_presenter::NinjaPresenter, player_presenter::PlayerPresenter},
};

pub struct CombatSystem {}

impl CombatSystem {
    pub fn start_combat(player_1: &mut Player, player_2: &mut Player) {
        while player_1.is_in_play() || player_2.is_in_play() {
            CombatSystem::player_turn(player_1, player_2);
            CombatSystem::player_turn(player_2, player_1);
        }
    }

    pub fn player_turn(attacking_player: &mut Player, defending_player: &mut Player) {
        PlayerPresenter::display_player_turn(&attacking_player);

        let attacking_ninja = NinjaPresenter::select_attacking_ninja(attacking_player.ninjas, defending_player.name);
        let mut defending_ninja = NinjaPresenter::select_defending_ninja(defending_player.ninjas, attacking_player.name);

        CombatSystem::combat_turn(attacking_ninja, &mut defending_ninja);
        CombatSystem::apply_damage(defending_player, defending_ninja);
        NinjaPresenter::display_ninja_status(&defending_ninja);
    }

    fn combat_turn(attacking_ninja: Ninja, defending_ninja: &mut Ninja) {
        defending_ninja.take_damage(attacking_ninja.attack);
    }

    fn apply_damage(defending_player: &mut Player, defending_ninja: Ninja) {
        let ninja_index = defending_player
            .ninjas
            .iter()
            .position(|&ninja| ninja.id == defending_ninja.id);
        defending_player.ninjas[ninja_index.unwrap()] = defending_ninja;
    }
}

#[cfg(test)]
mod controller_should {
    use super::*;

    #[test]
    fn combat_turn() {
        // Given
        let attacking_ninja = Ninja::default();
        let mut defending_ninja = Ninja::default();

        // Then
        CombatSystem::combat_turn(attacking_ninja, &mut defending_ninja);

        // When
        assert_eq!(75, defending_ninja.health);
    }

    #[test]
    fn apply_damage() {
        // Given
        let mut defending_player = Player::new_player_1();
        let defending_ninja = Ninja {
            id: defending_player.ninjas[0].id,
            name: defending_player.ninjas[0].name,
            health: 40,
            attack: defending_player.ninjas[0].attack,
        };

        // Then
        CombatSystem::apply_damage(&mut defending_player, defending_ninja);

        // When
        assert_eq!(40, defending_player.ninjas[0].health);
        assert_eq!(100, defending_player.ninjas[1].health);
        assert_eq!(100, defending_player.ninjas[2].health);
        assert_eq!(100, defending_player.ninjas[3].health);
    }
}
