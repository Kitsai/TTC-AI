use crate::tic_tac_toe;


pub trait Agent {
    fn evaluate(&self, state: tic_tac_toe::TicTacToe) -> isize;
    fn choose_move(&self, vec: Vec<(usize,usize)>, state: tic_tac_toe::TicTacToe) -> tic_tac_toe::TicTacToeTypes;
}
