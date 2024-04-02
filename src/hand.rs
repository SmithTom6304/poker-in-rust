use crate::card::Card;

struct Hand {
    cards: [Card; 2],
}

impl Hand {
    pub fn new(cards: [Card; 2]) -> Self {
        Hand { cards }
    }
}
