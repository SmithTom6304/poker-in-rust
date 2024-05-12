use crate::{hand::Hand, pot::Pot};

pub struct Player {
    pub hand: Hand,
    pub chips: u32,
}

pub struct FoldedPlayer {
    pub chips: u32,
}

impl Player {
    pub fn fold(self) -> FoldedPlayer {
        FoldedPlayer { chips: self.chips }
    }

    pub fn bet(&mut self, amount: u32, pot: &mut Pot) -> Result<(), String> {
        if amount < pot.minimum_bet {
            return Err(format!(
                "Bet amount ({}) is less than minimum bet ({})",
                amount, pot.minimum_bet
            ));
        }

        if self.chips < amount {
            return Err(format!(
                "Bet amount ({}) is greater than player chips ({})",
                amount, self.chips
            ));
        }

        self.chips -= amount;
        pot.chips += amount;
        Ok(())
    }
}

impl FoldedPlayer {
    pub fn deal_in(self, hand: Hand) -> Player {
        Player {
            chips: self.chips,
            hand,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{card::Card, hand::Hand, pot::Pot, rank::Rank, suit::Suit};

    use super::Player;

    fn has_a_hand_and_chips() -> Player {
        let hand = Hand::new([
            Card::new(Suit::Spade, Rank::Ace),
            Card::new(Suit::Heart, Rank::Queen),
        ]);
        let chips = 100;
        Player { hand, chips }
    }

    #[test]
    fn betting_reduces_chip_count() {
        let mut player = has_a_hand_and_chips();
        let mut pot = Pot::empty();
        assert!(player.bet(20, &mut pot).is_ok());
        assert_eq!(80, player.chips);
        assert_eq!(20, pot.chips);
    }

    #[test]
    fn cannot_bet_more_chips_than_player_has() {
        let mut player = has_a_hand_and_chips();
        let mut pot = Pot::empty();
        assert!(player.bet(300, &mut pot).is_err());
    }

    #[test]
    fn cannot_bet_less_than_minimum_pot_bet() {
        let mut player = has_a_hand_and_chips();
        let mut pot = Pot {
            chips: 50,
            minimum_bet: 10,
        };
        assert!(player.bet(5, &mut pot).is_err());
    }
}
