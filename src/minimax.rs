use crate::{eval::evaluate_state, game::{Move, State, Team}};

pub fn minimax(state: &State, depth:i32, maxi_player:Team, ) -> (Option<Move>, f32) {
    if state.is_over() {
        if state.winner().is_none() {
            return (None, 0.0);
        }
        if state.winner().unwrap() == maxi_player {
            return (None, 1000000000.0);
        } else {
            return (None, -1000000000.0);
        }
    }

    if depth == 0 {
        return (None, evaluate_state(state, maxi_player));
    }

    if state.current_team() == maxi_player {
        let mut max_value = -10000000000.0;
        let mut best_move:Move = state.possible_moves()[0];
        for mv in state.possible_moves() {
            let mut state_copy = state.clone();
            state_copy.perform(mv);
            let (_, value) = minimax(&state_copy, depth-1, maxi_player);
            if value > max_value {
                max_value = value;
                best_move = mv;
            }
        }
        return (Some(best_move), max_value)
    } else {
        let mut min_value = 10000000000.0;
        let mut best_move:Move = state.possible_moves()[0];
        for mv in state.possible_moves() {
            let mut state_copy = state.clone();
            state_copy.perform(mv);
            let (_, value) = minimax(&state_copy, depth-1, maxi_player);
            if value < min_value {
                min_value = value;
                best_move = mv;
            }
        }
        return (Some(best_move), min_value)
    }


    return (None, 0.0);
}