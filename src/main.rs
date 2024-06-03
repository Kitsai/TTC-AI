mod tic_tac_toe;
mod search_models;

use std::io;
use search_models::Agent;
use tic_tac_toe::{TicTacToe, TicTacToeTypes};

use crate::tic_tac_toe::IllegalMoveError;

fn main() {
    'menuloop: loop {
        println!("Bem vindo decida o que quer fazer?");
        println!("1 - normal (2 jogadores)");
        println!("2 - contra ia otima");
        println!("3 - contra ia de chance");
        println!("4 - contra ia com poda");
        println!("5 - 2 ias");
        println!("6 - exit");

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Erro ao ler linha.");


    }


}

fn read_play() -> (usize,usize) {
    println!("Posição x: ");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("Deu ruim");
    let x: usize = ans.trim().parse().unwrap();

    println!("Posição y: ");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("Deu ruim");
    let y: usize = ans.trim().parse().unwrap();
    (x,y)
}

fn player_move(game : &mut TicTacToe, player: u8) {
    println!();
    'move_loop: loop {
        println!("jogador {} sua vez! Escolha onde colocar o {}", player, TicTacToeTypes::from_u8(player).as_string());
        let player_move = read_play();
        match game.play(player_move.0, player_move.1, TicTacToeTypes::from_u8(player)) {
            Ok(new_state) =>  {
                *game = new_state;
                break 'move_loop;
            },
            Err(IllegalMoveError) => println!("Illegal move was made! Chose another point!\n"),
        }
    }
    println!()   
}

fn ai_move(game: &mut TicTacToe, symbol: TicTacToeTypes, agent: &dyn Agent) {

}

pub fn run_2players() {
    let mut game = TicTacToe::new();

    let mut player: u8 = 0;
    'gameloop: loop {
        game.print_board();
        player_move(&mut game, player + 1);
        
        match game.test_victory() {
            TicTacToeTypes::None => player = (player + 1)%2,
            _  => break 'gameloop,
        }



        player_move(&mut game, player + 1);

        match game.test_victory() {
            TicTacToeTypes::None => player = (player + 1)%2,
            _ => break 'gameloop
        }
    }
}

pub fn run_1player(ia: &dyn Agent) {

}

pub fn run_0player(a: &dyn Agent, b: &dyn Agent) {

}
