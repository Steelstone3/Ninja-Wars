use crate::{components::player_names::PlayerNames, entities::ninja::Ninja};
use inquire::Select;

pub struct NinjaPresenter {}

impl NinjaPresenter {
    pub fn select_attacking_ninja(ninjas: [Ninja; 4], winning_player_name: PlayerNames) -> Ninja {
        NinjaPresenter::select_ninja("Select Attacking Ninja:", ninjas, winning_player_name)
    }

    pub fn select_defending_ninja(ninjas: [Ninja; 4], winning_player_name: PlayerNames) -> Ninja {
        NinjaPresenter::select_ninja("Select Defending Ninja:", ninjas, winning_player_name)
    }

    pub fn display_ninja_status(ninja: &Ninja) {
        if ninja.health == 0 {
            println!("Ninja {} is dead!", ninja.name)
        } else {
            println!("{}", ninja.to_string());
        }
    }

    fn select_ninja(message: &str, ninjas: [Ninja; 4], winning_player_name: PlayerNames) -> Ninja {
        let mut alive_ninjas = ninjas.to_vec();
        alive_ninjas.retain(|&ninja| ninja.is_alive());

        Select::new(message, alive_ninjas)
            .prompt()
            .expect(format!("Player {} Wins!", winning_player_name.to_string()).as_str())
    }
}
