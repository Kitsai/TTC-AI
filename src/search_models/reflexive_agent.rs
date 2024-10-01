use crate::tic_tac_toe::{TicTacToe, TicTacToeTypes};

struct ReflexAgent {
    game: TicTacToe,
}

impl ReflexAgent {
    pub fn new(game: TicTacToe) -> Self {
        ReflexAgent {
            game
        }
    }
}

impl super::Agent for ReflexAgent {
    fn choose_move(&self, vec: Vec<(usize,usize)>, state: TicTacToe) -> (usize, usize) {
       (0,0) 
    }

    fn evaluate(&self, state: TicTacToe) -> isize {
        0
    }
}