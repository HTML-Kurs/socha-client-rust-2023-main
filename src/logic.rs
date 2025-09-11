use log::{info, debug};
use rand::seq::SliceRandom;

use socha_client_2023::{client::GameClientDelegate, eval::evaluate_state, game::{Move, State, Team}};

/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;



impl GameClientDelegate for OwnLogic {

    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        info!("Requested move");
        let mut chosen_move = *state.possible_moves()
            .choose(&mut rand::thread_rng())
            .expect("No move found!");

        if state.turn() <= 8 {
            return chosen_move;
        }

        let mut maximum_evaluation = 0.0;
        for pos_move in state.possible_moves() {
            // Simuliere den Zug pos_move
            let mut copyState = state.clone();
            copyState.perform(pos_move);
            let val = evaluate_state(&copyState, _my_team);
            if val > maximum_evaluation {
                maximum_evaluation = val;
                chosen_move = pos_move;
            }
        }

        info!("Chose move {}", chosen_move);
        chosen_move
    }

    fn on_update_state(&mut self, state: &State) {
        debug!("Board:\n{}", state.board());
    }
    
}
