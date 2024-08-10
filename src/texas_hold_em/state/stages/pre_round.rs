use std::fmt::Display;

use crate::{
    player::{Active, Folded, Player, PlayerId},
    Deck, Hand, Pot,
};

use super::pre_flop::PreFlop;

#[derive(Debug)]
pub struct PreRound {
    pub players: Vec<Player<Folded>>,
    pub pot: Pot,
    pub deck: Deck,
}

impl PreRound {
    pub fn new(players: u8) -> Result<Self, String> {
        if players < 2 {
            return Err("Game requires at least two players".to_string());
        }

        let pot = Pot::default();
        let mut deck = Deck::new().shuffle();

        let mut folded_players = vec![];
        let mut i = 0;
        while i < players {
            i += 1;
            folded_players.push(Self::deal_player(PlayerId(i), &mut deck))
        }

        Ok(Self {
            players: folded_players,
            pot,
            deck,
        })
    }

    pub fn start_round(self) -> PreFlop {
        self.print_stage_info();
        let mut deck = Deck::new().shuffle();
        let active_players = self
            .players
            .into_iter()
            .map(|player| PreRound::deal_player_in(player, &mut deck))
            .collect();
        let folded_players = vec![];
        let pot = Pot::default();
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

    fn print_stage_info(&self) {
        println!("{}", self);
        for player in self.players.iter() {
            println!("{}", player)
        }
    }

    fn deal_player(id: PlayerId, deck: &mut Deck) -> Player<Folded> {
        let cards = [deck.draw().unwrap(), deck.draw().unwrap()];
        Player::<Folded>::new(id, Hand::new(cards), 100)
    }
}

impl Display for PreRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pre-Round - Players: {}", self.players.len())
    }
}
