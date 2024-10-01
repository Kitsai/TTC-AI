use crate::tic_tac_toe::{TicTacToe,TicTacToeTypes};
use super::Agent;

struct MinimaxAgent {
    game: TicTacToe,
}

impl MinimaxAgent {
    fn value(&self, state: TicTacToe) {

    }

    fn min(&self, state: TicTacToe, player: u8) {
        let valid_moves = state.legal_actions();
        let valid_states: Vec<TicTacToe> = Vec::new();
        for i in valid_moves {
            valid_states.append(self.get_state_from_move())
        }
    }

    fn max(&self) {
        
    }

    fn get_state_from_move(p_move: (usize,usize), state:TicTacToe, symbol: TicTacToeTypes) -> TicTacToe {
        state.play(p_move.0, p_move.1, symbol).expect("faio")
    }
}

impl Agent for MinimaxAgent {
    fn choose_move(&self, vec: Vec<(usize,usize)>, state: TicTacToe) -> (usize, usize) {
        
    }

    fn evaluate(&self, state: TicTacToe) -> isize {
        
    }
}