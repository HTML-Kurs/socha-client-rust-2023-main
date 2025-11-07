use log::{info, debug};
use rand::seq::SliceRandom;

use crate::game::State;
use crate::game::Move;
use crate::game::Team;

fn calc_fish_diff(state: &State, _my_team:Team) -> f32 {
    return (state.fish(_my_team) as f32 - state.fish(_my_team.opponent()) as f32) as f32;
}

fn calc_fish_in_one_move(state: &State, _my_team: Team) -> f32 {
    let mut fishCount= 0.0;
    for mv in state.possible_moves_for_team(_my_team) {
        fishCount += get_fish_of_dest_move(state, _my_team, mv);
    }

    for mv in state.possible_moves_for_team(_my_team.opponent()) {
        fishCount -= get_fish_of_dest_move(state, _my_team, mv);
    }


    // TODO: Die Fische die der Gegner in 1 Zug erreichen kann abziehen
    return fishCount;
}

// Hausaufgabe: Anzahl der Pinguine Ermitteln, die noch einen Zug machen können (Erstmal nur für das eigene team)
fn calc_movable_penguins(state: &State, _my_team: Team) -> f32 {
    let mut pengus = 0.0;
    for pengu in state.team_pieces(_my_team) {
        if state.board().possible_moves_from(pengu.0).next().is_some() {
            pengus+=1.0;
        }
    }
    for pengu in state.team_pieces(_my_team.opponent()) {
        if state.board().possible_moves_from(pengu.0).next().is_some() {
            pengus-=1.0;
        }
    }
    return pengus;
}

// Brute Force -> nächste Züge simulieren (Da wollen wir hin MiniMax)
// Anzahl der Möglichen Züge für den Gegner und für mich wenn ich den Zug mache
// wie viele fische kann ich von feld wo ich hinziehe aus erreichen?
// Fische in unmittelbarer nähe des feldes
pub fn get_fish_of_dest_move(state: &State, _my_team: Team, m:Move) -> f32 {
    let fieldCord = m.to();
    let realField = state.board()[fieldCord];
    let fishNum = realField.fish();
    return fishNum as f32;
}

pub fn evaluate_state(state: &State, _my_team:Team) -> f32 {
    return 
        calc_fish_diff(state, _my_team)        * 1.0 +
        calc_fish_in_one_move(state, _my_team) * 0.2 +
        calc_movable_penguins(state, _my_team) * 3.0;
}
