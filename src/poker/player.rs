use std::fmt::Display;

use crate::{Hand, Pot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player<S: PlayerState> {
    pub id: PlayerId,
    pub hand: Hand,
    pub chips: u32,
    pub state: S,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Active {
    pub chips_bet_in_stage: u32,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Folded {}

pub trait PlayerState {}

impl PlayerState for Active {}
impl PlayerState for Folded {}

impl Player<Active> {
    pub fn new(id: PlayerId, hand: Hand, chips: u32) -> Self {
        Player::<Active> {
            id,
            hand,
            chips,
            state: Active::default(),
        }
    }

    pub fn fold(self) -> Player<Folded> {
        Player::<Folded> {
            id: self.id,
            hand: self.hand,
            chips: self.chips,
            state: Folded::default(),
        }
    }

    pub fn bet(&mut self, amount: u32, pot: &mut Pot) -> Result<(), String> {
        if amount < (pot.minimum_bet - self.state.chips_bet_in_stage) {
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
        self.state.chips_bet_in_stage += amount;
        pot.chips += amount;
        Ok(())
    }
}

impl Display for Player<Active> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player {} - Hand: {} - Chips: {}",
            self.id, self.hand, self.chips
        )
    }
}

impl Player<Folded> {
    pub fn new(id: PlayerId, hand: Hand, chips: u32) -> Self {
        Player::<Folded> {
            id,
            hand,
            chips,
            state: Folded::default(),
        }
    }

    pub fn deal_in(self, hand: Hand) -> Player<Active> {
        Player::<Active> {
            id: self.id,
            chips: self.chips,
            hand,
            state: Active::default(),
        }
    }
}

impl Display for Player<Folded> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {} - Chips: {}", self.id, self.chips)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PlayerId(pub u8);
impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Hand, Pot, Rank, Suit};

    use super::{Active, Player, PlayerId};

    fn create_test_player(id: u8) -> Player<Active> {
        let hand = Hand::new([
            Card::new(Suit::Spade, Rank::Ace),
            Card::new(Suit::Heart, Rank::Queen),
        ]);
        let chips = 100;
        Player::<Active>::new(PlayerId(id), hand, chips)
    }

    #[test]
    fn betting_reduces_chip_count() {
        let mut player = create_test_player(0);
        let mut pot = Pot::default();
        assert!(player.bet(20, &mut pot).is_ok());
        assert_eq!(80, player.chips);
        assert_eq!(20, pot.chips);
    }

    #[test]
    fn cannot_bet_more_chips_than_player_has() {
        let mut player = create_test_player(0);
        let mut pot = Pot::default();
        assert!(player.bet(300, &mut pot).is_err());
    }

    #[test]
    fn cannot_bet_less_than_minimum_pot_bet() {
        let mut player = create_test_player(0);
        let mut pot = Pot {
            chips: 50,
            minimum_bet: 10,
        };
        assert!(player.bet(5, &mut pot).is_err());
    }

    #[test]
    fn raising_increases_minimum_bet() {
        let mut player = create_test_player(0);
        let mut pot = Pot {
            chips: 50,
            minimum_bet: 10,
        };
        _ = player.bet(20, &mut pot);
        assert_eq!(20, pot.minimum_bet)
    }
}
