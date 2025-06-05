use log::{info, debug};
use rand::seq::SliceRandom;

use socha_client_2023::{client::GameClientDelegate, game::{Move, Team, State}};

/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;


// Brute Force -> nächste Züge simulieren (Da wollen wir hin MiniMax)
// Anzahl der Möglichen Züge für den Gegner und für mich wenn ich den Zug mache
// wie viele fische kann ich von feld wo ich hinziehe aus erreichen?
// Fische in unmittelbarer nähe des feldes
// 
fn evaluate_move(state: &State, _my_team: Team, m:Move) -> f32 {
    let fieldCord = m.to();
    let realField = state.board()[fieldCord];
    let fishNum = realField.fish();
    return fishNum as f32;
}

impl GameClientDelegate for OwnLogic {


    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        info!("Requested move");
        let mut chosen_move = *state.possible_moves()
            .choose(&mut rand::thread_rng())
            .expect("No move found!");



        if state.turn() <= 8 {
            return chosen_move;
        }
        let mut maximum_fish = 0.0;
        for pos_move in state.possible_moves() {
           let val = evaluate_move(state, _my_team, pos_move);
            if val > maximum_fish {
                maximum_fish = val;
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
