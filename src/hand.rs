use std::fmt::Display;

use crate::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hand {
    pub cards: [Card; 2],
}

impl Hand {
    pub fn new(cards: [Card; 2]) -> Self {
        Hand { cards }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.cards[0], self.cards[1])
    }
}
