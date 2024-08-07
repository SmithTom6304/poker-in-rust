use std::fmt::Display;

use crate::player::{Active, Player};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Pot {
    pub chips: u32,
    pub minimum_bet: u32,
}

impl Pot {
    /// Distributes pot winnings to a list of players.
    ///
    /// Players should be ordered by distance to dealer to account for splitting of uneven chip counts.
    pub fn deal_winnings(&mut self, mut winners: Vec<&mut Player<Active>>) {
        let division = self.chips / winners.len() as u32;
        winners
            .iter_mut()
            .for_each(|winner| winner.chips += division);

        // Distribute any extra chips based on position to the dealer, ie. order in the vec
        let odd_chips = self.chips % winners.len() as u32;
        for i in 0..odd_chips {
            winners[i as usize].chips += 1;
        }

        self.chips = 0;
        self.minimum_bet = 0;
    }

    pub fn new(chips: u32, minimum_bet: u32) -> Self {
        Pot { chips, minimum_bet }
    }
}

impl Display for Pot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pot: {} chips, minimum bet {}",
            self.chips, self.minimum_bet
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        player::{Active, Player, PlayerId},
        Deck, Hand,
    };

    use super::Pot;

    fn create_test_player(deck: &mut Deck, chips: u32) -> Player<Active> {
        let hand = Hand::new([
            deck.draw().expect("Deck was empty"),
            deck.draw().expect("Deck was empty"),
        ]);
        Player::<Active>::new(PlayerId(0), hand, chips)
    }

    #[test]
    fn can_deal_its_winnings_to_players() {
        let mut deck = Deck::new();
        let player_chips = 100;
        let pot_chips = 50;

        let mut players = [
            create_test_player(&mut deck, player_chips),
            create_test_player(&mut deck, player_chips),
        ];

        let mut pot = Pot {
            chips: pot_chips,
            minimum_bet: 0,
        };
        pot.deal_winnings(players.iter_mut().collect());

        let pot_chips_per_player = pot_chips / players.len() as u32;
        assert!(players
            .iter()
            .all(|player| player_chips + pot_chips_per_player == player.chips));
        assert_eq!(0, pot.chips);
    }

    #[test]
    fn distributes_extra_chips_by_player_order() {
        let mut deck = Deck::new();
        let player_chips = 5;
        let pot_chips = 5;

        let mut players = [
            create_test_player(&mut deck, player_chips),
            create_test_player(&mut deck, player_chips),
            create_test_player(&mut deck, player_chips),
        ];

        let mut pot = Pot {
            chips: pot_chips,
            minimum_bet: 0,
        };
        pot.deal_winnings(players.iter_mut().collect());

        // 5 into 3 does not go.. so divide based on player order
        assert_eq!(7, players[0].chips);
        assert_eq!(7, players[1].chips);
        assert_eq!(6, players[2].chips);
    }
}
