use std::fmt::Display;

use crate::{hand::Hand, pot::Pot};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PlayerId(pub u8);
impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub hand: Hand,
    pub chips: u32,
}

#[derive(Debug)]
pub struct FoldedPlayer {
    pub id: PlayerId,
    pub chips: u32,
}

impl Player {
    pub fn fold(self) -> FoldedPlayer {
        FoldedPlayer {
            chips: self.chips,
            id: self.id,
        }
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

        if amount > pot.minimum_bet {
            pot.minimum_bet = amount;
        }

        self.chips -= amount;
        pot.chips += amount;
        Ok(())
    }
}

impl FoldedPlayer {
    pub fn deal_in(self, hand: Hand) -> Player {
        Player {
            id: self.id,
            chips: self.chips,
            hand,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{card::Card, hand::Hand, pot::Pot, rank::Rank, suit::Suit};

    use super::{Player, PlayerId};

    fn has_a_hand_and_chips(id: u8) -> Player {
        let hand = Hand::new([
            Card::new(Suit::Spade, Rank::Ace),
            Card::new(Suit::Heart, Rank::Queen),
        ]);
        let chips = 100;
        Player {
            id: PlayerId(id),
            hand,
            chips,
        }
    }

    #[test]
    fn betting_reduces_chip_count() {
        let mut player = has_a_hand_and_chips(0);
        let mut pot = Pot::empty();
        assert!(player.bet(20, &mut pot).is_ok());
        assert_eq!(80, player.chips);
        assert_eq!(20, pot.chips);
    }

    #[test]
    fn cannot_bet_more_chips_than_player_has() {
        let mut player = has_a_hand_and_chips(0);
        let mut pot = Pot::empty();
        assert!(player.bet(300, &mut pot).is_err());
    }

    #[test]
    fn cannot_bet_less_than_minimum_pot_bet() {
        let mut player = has_a_hand_and_chips(0);
        let mut pot = Pot {
            chips: 50,
            minimum_bet: 10,
        };
        assert!(player.bet(5, &mut pot).is_err());
    }

    #[test]
    fn raising_increases_minimum_bet() {
        let mut player = has_a_hand_and_chips(0);
        let mut pot = Pot {
            chips: 50,
            minimum_bet: 10,
        };
        _ = player.bet(20, &mut pot);
        assert_eq!(20, pot.minimum_bet)
    }
}
