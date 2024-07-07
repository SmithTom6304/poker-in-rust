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
        println!("SHOWDOWN");
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
}
