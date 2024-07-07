use crate::card::Card;

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: [Card; 2],
}

impl Hand {
    pub fn new(cards: [Card; 2]) -> Self {
        Hand { cards }
    }
}
