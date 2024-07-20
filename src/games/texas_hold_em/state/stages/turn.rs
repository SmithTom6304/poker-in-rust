use std::fmt::Display;

use crate::{
    card::Card,
    deck::Deck,
    games::texas_hold_em::state::{
        advancement::Advancement,
        game_loop::{GameLoop, StageOutcome},
    },
    player::{Active, Folded, Player},
    pot::Pot,
};

use super::{finished::Finished, river::River};

#[derive(Debug)]
pub struct Turn {
    pub active_players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub pot: Pot,
    pub deck: Deck,
    pub cards: [Card; 4],
}

impl Turn {
    pub fn advance(mut self) -> Advancement<River> {
        self.print_pre_round_info();
        let mut deck = self.deck;
        let cards = [
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            deck.draw().unwrap(),
        ];

        self.deck = deck;
        let game_loop = self.create_game_loop();
        let stage_outcome = game_loop.do_stage();

        match stage_outcome {
            StageOutcome::NextStage(game_loop) => Advancement::NextStage(River {
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

    fn print_pre_round_info(&self) {
        println!("{}", self);
        for player in self.active_players.iter() {
            println!("{}", player)
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Turn - Players: {} - Pot: {} - Cards: {} {} {} {}",
            self.active_players.len(),
            self.pot.chips,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3]
        )
    }
}
