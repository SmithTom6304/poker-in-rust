use crate::player::Player;

pub struct Pot {
    pub chips: u32,
}

impl Pot {
    /// Distributes pot winnings to a list of players.
    ///
    /// Players should be ordered by distance to dealer to account for splitting of uneven chip counts.
    ///
    /// Returns an empty pot.
    pub fn deal_winnings(self, mut winners: Vec<&mut Player>) -> Pot {
        let division = self.chips / winners.len() as u32;
        winners
            .iter_mut()
            .for_each(|winner| winner.chips += division);

        // Distribute any extra chips based on position to the dealer, ie. order in the vec
        let odd_chips = self.chips % winners.len() as u32;
        for i in 0..odd_chips {
            winners[i as usize].chips += 1;
        }

        Pot { chips: 0 }
    }
}

#[cfg(test)]
mod tests {

    use crate::{deck::Deck, hand::Hand, player::Player};

    use super::Pot;

    fn create_test_player(deck: &mut Deck, chips: u32) -> Player {
        let hand = Hand::new([
            deck.draw().expect("Deck was empty"),
            deck.draw().expect("Deck was empty"),
        ]);
        Player { hand, chips }
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

        let pot = Pot { chips: pot_chips };
        let emptied_pot = pot.deal_winnings(players.iter_mut().collect());

        let pot_chips_per_player = pot_chips / players.len() as u32;
        assert!(players
            .iter()
            .all(|player| player_chips + pot_chips_per_player == player.chips));
        assert_eq!(0, emptied_pot.chips);
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

        let pot = Pot { chips: pot_chips };
        _ = pot.deal_winnings(players.iter_mut().collect());

        // 5 into 3 does not go.. so divide based on player order
        assert_eq!(7, players[0].chips);
        assert_eq!(7, players[1].chips);
        assert_eq!(6, players[2].chips);
    }
}
