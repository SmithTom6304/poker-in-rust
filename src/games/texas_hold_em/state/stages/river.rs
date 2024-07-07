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

use super::{finished::Finished, showdown::Showdown};

#[derive(Debug)]
pub struct River {
    pub active_players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub pot: Pot,
    pub deck: Deck,
    pub cards: [Card; 5],
}

impl River {
    pub fn advance(mut self) -> Advancement<Showdown> {
        println!("RIVER");
        let deck = self.deck;
        let cards = self.cards;

        self.deck = deck;
        let game_loop = self.create_game_loop();
        let stage_outcome = game_loop.do_stage();

        match stage_outcome {
            StageOutcome::NextStage(game_loop) => Advancement::NextStage(Showdown {
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
}
