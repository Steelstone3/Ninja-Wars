use crate::entities::ninja::Ninja;
use inquire::Select;

pub struct NinjaPresenter {}

impl NinjaPresenter {
    pub fn select_attacking_ninja(ninjas: [Ninja; 4]) -> Ninja {
        NinjaPresenter::select_ninja("Select Attacking Ninja:", ninjas)
    }

    pub fn select_defending_ninja(ninjas: [Ninja; 4]) -> Ninja {
        NinjaPresenter::select_ninja("Select Defending Ninja:", ninjas)
    }

    pub fn display_ninja_status(ninja: &Ninja) {
        if ninja.health == 0 {
            println!("Ninja {} is dead!", ninja.name)
        } else {
            println!("{}", ninja.to_string());
        }
    }

    fn select_ninja(message: &str, ninjas: [Ninja; 4]) -> Ninja {
        let mut alive_ninjas = ninjas.to_vec();
        alive_ninjas.retain(|&ninja| ninja.is_alive());

        Select::new(message, alive_ninjas).prompt().unwrap()
    }
}
