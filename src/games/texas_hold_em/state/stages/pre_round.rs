use crate::{
    deck::Deck,
    hand::Hand,
    player::{Active, Folded, Player},
    pot::Pot,
};

use super::pre_flop::PreFlop;

#[derive(Debug)]
pub struct PreRound {
    pub players: Vec<Player<Folded>>,
}

impl PreRound {
    pub fn new(players: Vec<Player<Folded>>) -> Self {
        Self { players }
    }

    pub fn start_round(self) -> PreFlop {
        let mut deck = Deck::new().shuffle();
        let active_players = self
            .players
            .into_iter()
            .map(|player| PreRound::deal_player_in(player, &mut deck))
            .collect();
        let folded_players = vec![];
        let pot = Pot::empty();
        PreFlop {
            active_players,
            folded_players,
            pot,
            deck,
        }
    }

    fn deal_player_in(player: Player<Folded>, deck: &mut Deck) -> Player<Active> {
        let cards = [deck.draw().unwrap(), deck.draw().unwrap()];
        player.deal_in(Hand::new(cards))
    }
}
