use std::fmt;

use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Clone, Copy, Hash)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_string = match self {
            Suit::Club => '\u{2663}',
            Suit::Diamond => '\u{2666}',
            Suit::Heart => '\u{2665}',
            Suit::Spade => '\u{2660}',
        };
        write!(f, "{}", rank_string)
    }
}

#[cfg(test)]
mod tests {
    use super::Suit;
    use rstest::rstest;

    #[rstest]
    #[case(Suit::Club, r#"♣"#)]
    #[case(Suit::Diamond, r#"♦"#)]
    #[case(Suit::Heart, r#"♥"#)]
    #[case(Suit::Spade, r#"♠"#)]
    fn can_be_displayed(#[case] suit: Suit, #[case] expected_string: String) {
        assert_eq!(expected_string, suit.to_string())
    }
}
