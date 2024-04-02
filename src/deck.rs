use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![];
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }

    pub fn shuffle(self) -> Self {
        let mut cards = Vec::from(self.cards);
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::Deck;

    #[test]
    fn has_52_cards_at_creation() {
        let deck = Deck::new();
        assert_eq!(52, deck.cards.len())
    }

    #[test]
    fn can_draw_card_if_not_empty() {
        let mut deck = Deck::new();
        assert!(deck.draw().is_some())
    }

    #[test]
    fn cannot_draw_card_if_empty() {
        let mut deck = Deck::new();
        deck.cards.clear();
        assert!(deck.draw().is_none())
    }

    // Can theoretically fail - but not likely
    #[test]
    fn can_be_shuffled() {
        let deck = Deck::new();
        let original_order = deck.cards.to_vec();
        let shuffled_order = deck.shuffle().cards;
        assert!(original_order
            .iter()
            .zip(shuffled_order.iter())
            .any(|pair| pair.0 != pair.1))
    }
}
