use std::fmt::Display;

use super::super::{
    advancement::Advancement,
    game_loop::{GameLoop, StageOutcome},
};
use crate::{
    player::{Active, Folded, Player},
    Card, Deck, Pot,
};

use super::{finished::Finished, turn::Turn};

#[derive(Debug)]
pub struct Flop {
    pub active_players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub pot: Pot,
    pub deck: Deck,
    pub cards: [Card; 3],
}

impl Flop {
    pub fn advance(mut self) -> Advancement<Turn> {
        self.print_stage_info();
        let mut deck = self.deck;
        let cards = [
            self.cards[0],
            self.cards[1],
            self.cards[2],
            deck.draw().unwrap(),
        ];

        self.deck = deck;
        let game_loop = self.create_game_loop();
        let stage_outcome = game_loop.do_stage();

        match stage_outcome {
            StageOutcome::NextStage(game_loop) => Advancement::NextStage(Turn {
                active_players: game_loop.active_players,
                folded_players: game_loop.folded_players,
                pot: game_loop.pot,
                deck: self.deck,
                cards,
            }),
            StageOutcome::Finished(game_loop) => Advancement::Finished(Finished {
                active_players: game_loop.active_players,
                folded_players: game_loop.folded_players,
                pot: game_loop.pot,
                deck: self.deck,
            }),
        }
    }

    fn create_game_loop(&self) -> GameLoop {
        let active_players = self.active_players.clone();
        let folded_players = self.folded_players.clone();
        let button_index = active_players.len() - 1; // TODO Bump each round
        let current_player_index = 0;
        let pot = self.pot;
        GameLoop {
            active_players,
            folded_players,
            button_index,
            current_player_index,
            pot,
        }
    }

    fn print_stage_info(&self) {
        println!("{}", self);
        for player in self.active_players.iter() {
            println!("{}", player)
        }
    }
}

impl Display for Flop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Flop - Players: {} - Pot: {} - Cards: {} {} {}",
            self.active_players.len(),
            self.pot.chips,
            self.cards[0],
            self.cards[1],
            self.cards[2]
        )
    }
}
