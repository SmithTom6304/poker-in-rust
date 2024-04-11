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
        match amount > self.chips {
            true => Err("Player does not have enough chips".to_string()),
            false => {
                self.chips -= amount;
                pot.chips += amount;
                Ok(())
            }
        }
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
        let mut pot = Pot { chips: 0 };
        assert!(player.bet(20, &mut pot).is_ok());
        assert_eq!(80, player.chips);
        assert_eq!(20, pot.chips);
    }

    #[test]
    fn cannot_bet_more_chips_than_player_has() {
        let mut player = has_a_hand_and_chips();
        let mut pot = Pot { chips: 0 };
        assert!(player.bet(300, &mut pot).is_err());
    }
}
