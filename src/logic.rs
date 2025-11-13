use core::f32;

use log::{info, debug};
use rand::seq::SliceRandom;

use socha_client_2023::{client::GameClientDelegate, eval::evaluate_state, game::{Move, State, Team}, minimax::minimax};

/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;



impl GameClientDelegate for OwnLogic {

    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        info!("Requested move");
        let (my_move, _) = minimax(state, 3, _my_team, f32::MIN, f32::MAX);
        let chosen_move = my_move.unwrap();
        info!("Chose move {}", chosen_move);
        chosen_move
    }

    fn on_update_state(&mut self, state: &State) {
        debug!("Board:\n{}", state.board());
    }
    
}
