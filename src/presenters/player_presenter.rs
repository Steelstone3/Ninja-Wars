use crate::entities::player::Player;

pub struct PlayerPresenter {}

impl PlayerPresenter {
    pub fn display_player_turn(player: &Player) {
        println!("{}'s Turn", player.name)
    }
}
