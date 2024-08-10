use std::fmt::Display;

use crate::{
    player::{Active, Folded, Player},
    texas_hold_em::evaluation::evaluator::{Evaluator, HandVal},
    Card, Deck, Pot,
};

use super::finished::Finished;

pub struct Showdown {
    pub active_players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub pot: Pot,
    pub deck: Deck,
    pub cards: [Card; 5],
}

#[derive(Copy, Clone)]
pub struct PlayerRankPair<'player> {
    pub player: &'player Player<Active>,
    pub rank: HandVal,
}

impl Showdown {
    pub fn finish(mut self, evaluator: Box<dyn Evaluator>) -> Finished {
        self.print_stage_info();

        self.fold_losers(evaluator);

        let active_players = self.active_players;
        let folded_players = self.folded_players;
        let pot = self.pot;
        let deck = self.deck.clone();

        Finished {
            active_players,
            folded_players,
            pot,
            deck,
        }
    }

    pub fn determine_player_ranks(&self, evaluator: Box<dyn Evaluator>) -> Vec<PlayerRankPair> {
        self.active_players
            .iter()
            .map(|player| Showdown::determine_player_rank(self.cards, player, &evaluator))
            .collect::<Vec<PlayerRankPair>>()
    }

    fn fold_losers(&mut self, evaluator: Box<dyn Evaluator>) {
        let player_ranks = self.determine_player_ranks(evaluator);
        let high_score = player_ranks.iter().map(|p| p.rank).max().unwrap();

        let losers = player_ranks
            .iter()
            .filter(|p| p.rank < high_score)
            .map(|r| *r.player)
            .collect::<Vec<Player<Active>>>();

        self.active_players
            .retain(|player| !losers.contains(player));
        let mut losers = losers
            .iter()
            .map(|loser| loser.fold())
            .collect::<Vec<Player<Folded>>>();
        self.folded_players.append(&mut losers);
    }

    pub fn determine_player_rank<'player>(
        community_cards: [Card; 5],
        player: &'player Player<Active>,
        evaluator: &Box<dyn Evaluator>,
    ) -> PlayerRankPair<'player> {
        let cards = [
            player.hand.cards[0],
            player.hand.cards[1],
            community_cards[0],
            community_cards[1],
            community_cards[2],
            community_cards[3],
            community_cards[4],
        ];
        let rank = evaluator.evaluate_hand(&cards);
        PlayerRankPair { player, rank }
    }

    fn print_stage_info(&self) {
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
