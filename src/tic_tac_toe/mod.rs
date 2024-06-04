use core::fmt;

use crate::search_models::Agent;

#[derive(Clone, Copy, Debug)]
pub struct IllegalMoveError;
pub struct DrawError;

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Invalid move was done")
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum TicTacToeTypes {
    None = 0, // 000
    O = 2, // 010
    X = 1, // 001
}

impl TicTacToeTypes {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => TicTacToeTypes::None,
            1 => TicTacToeTypes::X,
            2 => TicTacToeTypes::O,
            _ => panic!("Unknown value"),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            TicTacToeTypes::None => " ".to_string(),
            TicTacToeTypes::O => "O".to_string(),
            TicTacToeTypes::X => "X".to_string(),
        }
    }
}

pub struct TicTacToeMove(usize,usize);

#[derive(Clone, Copy)]
pub struct TicTacToe {
    pub board: [[TicTacToeTypes;3];3],
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [[TicTacToeTypes::None;3];3],
        }
    }

    fn print_separator(&self) {
        println!("---+---+---");
    }

    fn print_line(&self, idx: usize) {
        println!(" {}",self.board[idx][0].as_string() + " | " + &self.board[idx][1].as_string() + " | " + &self.board[idx][2].as_string());
    }

    pub fn print_board(&self) {
        self.print_line(0);
        self.print_separator();
        self.print_line(1);
        self.print_separator();
        self.print_line(2);
    }

    pub fn test_victory(&self) -> Result<TicTacToeTypes, DrawError> {

        let mut total_d1: u8 = 3;
        let mut total_d2: u8 = 3;
        let mut none_false: bool = false;

        for i in 0..3 {
            let mut total_l: u8 = 3;
            let mut total_c: u8 = 3;
            for j in 0..3 {
                total_l &= self.board[i][j] as u8;
                total_c &= self.board[j][i] as u8;
            }

            if total_l != 0 {
                return Ok(TicTacToeTypes::from_u8(total_l)) ;
            }

            if total_c != 0 {
                return Ok(TicTacToeTypes::from_u8(total_c));
            }

            total_d1 &= self.board[i][i] as u8;
            total_d2 &= self.board[2-i][i] as u8;
        }

        if total_d1 != 0 {
            return Ok(TicTacToeTypes::from_u8(total_d1));
        }

        if total_d2 != 0 {
            return Ok(TicTacToeTypes::from_u8(total_d2));
        }

        if !none_false {
            return Err(DrawError);
        }

        Ok(TicTacToeTypes::None)
    }

    pub fn play(&self, x: usize, y: usize, symbol: TicTacToeTypes) -> Result<TicTacToe, IllegalMoveError> {
        if self.board[2-y][x] != TicTacToeTypes::None  || x > 2 || y > 2 {
            return Err(IllegalMoveError);
        }

        let mut t = self.clone();
        t.board[2-y][x] = symbol;
        Ok(t)
    }

    pub fn legal_actions(&self) -> Vec<(usize,usize)> {
        let mut d: Vec<(usize,usize)> = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == TicTacToeTypes::None {
                    d.push((i,j));
                }
            }
        }

        d
    }

}