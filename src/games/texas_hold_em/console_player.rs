use std::io;

use super::{
    game::Game,
    player_driver::{Move, PlayerDriver},
};

pub struct ConsolePlayer {}

impl PlayerDriver for ConsolePlayer {
    fn determine_move(game: &Game) -> Move {
        println!("Press F to fold, C to call, R to raise by 10");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let player_move = match answer.to_ascii_lowercase().trim() {
            "f" => Move::Fold,
            "c" => Move::Call,
            "r" => Move::Raise {
                amount: game.pot.minimum_bet + 10,
            },
            _ => Move::Fold,
        };

        player_move
    }
}
