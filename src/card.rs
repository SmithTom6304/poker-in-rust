use std::fmt;

use crate::{rank::Rank, suit::Suit};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use crate::{rank::Rank, suit::Suit};

    use super::Card;
    use rstest::rstest;

    #[rstest]
    #[case(Card::new(Suit::Spade, Rank::Ace), r#"A♠"#)]
    fn can_be_displayed(#[case] card: Card, #[case] expected_string: String) {
        assert_eq!(expected_string, card.to_string())
    }
}
