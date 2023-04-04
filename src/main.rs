use entities::player::Player;
use systems::combat_system::CombatSystem;

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut player_1 = Player::new_player_1();
    let mut player_2 = Player::new_player_2();

    CombatSystem::start_combat(&mut player_1, &mut player_2);
}
