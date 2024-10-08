use std::fmt::Display;

use crate::{
    player::{Active, Folded, Player},
    Deck, Pot,
};

use super::pre_round::PreRound;

#[derive(Debug)]
pub struct Finished {
    pub active_players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub pot: Pot,
    pub deck: Deck,
}

impl Finished {
    pub fn payout(self) -> PreRound {
        self.print_stage_info();
        let mut winners = self.active_players.clone();
        let mut pot = self.pot;
        pot.deal_winnings(winners.iter_mut().collect());
        let players = self
            .folded_players
            .into_iter()
            .chain(Finished::fold_active_players(winners).into_iter())
            .collect();
        PreRound {
            players,
            pot,
            deck: self.deck,
        }
    }

    fn fold_active_players(active_players: Vec<Player<Active>>) -> Vec<Player<Folded>> {
        active_players
            .into_iter()
            .map(|player| player.fold())
            .collect()
    }

    fn print_stage_info(&self) {
        println!("{}", self);
        println!(
            "Players {} split a pot of {}",
            self.active_players
                .iter()
                .map(|player| player.id.to_string())
                .reduce(|acc, id| format!("{acc}, {id}"))
                .unwrap_or_default(),
            self.pot.chips
        )
    }
}

impl Display for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Round finished")
    }
}
