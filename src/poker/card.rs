use std::fmt;

use crate::{Rank, Suit};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
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
    use crate::{Rank, Suit};

    use super::Card;
    use rstest::rstest;

    #[rstest]
    #[case(Card::new(Suit::Spade, Rank::Ace), r#"A♠"#)]
    fn can_be_displayed(#[case] card: Card, #[case] expected_string: String) {
        assert_eq!(expected_string, card.to_string())
    }
}
