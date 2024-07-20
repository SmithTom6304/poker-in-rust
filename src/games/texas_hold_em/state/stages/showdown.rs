use std::fmt::Display;

use crate::{
    card::Card,
    deck::Deck,
    player::{Active, Folded, Player},
    pot::Pot,
};

use super::finished::Finished;

pub struct Showdown {
    pub active_players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub pot: Pot,
    pub deck: Deck,
    pub cards: [Card; 5],
}

impl Showdown {
    pub fn do_showdown(self) -> Finished {
        self.print_pre_round_info();
        let active_players = self.active_players;
        let folded_players = self.folded_players;
        let pot = self.pot;
        let deck = self.deck;

        // Fold all non-winners
        Finished {
            active_players,
            folded_players,
            pot,
            deck,
        }
    }

    fn print_pre_round_info(&self) {
        println!("{}", self);
        for player in self.active_players.iter() {
            println!("{}", player)
        }
    }
}

impl Display for Showdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Showdown! - Players: {} - Pot: {} - Cards: {} {} {} {} {}",
            self.active_players.len(),
            self.pot.chips,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4]
        )
    }
}
