use std::fmt;

use strum_macros::EnumIter;

#[derive(PartialEq, PartialOrd, EnumIter, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_string = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", rank_string)
    }
}

#[cfg(test)]
mod tests {
    use super::Rank;
    use rstest::rstest;

    #[rstest]
    #[case(Rank::Two, "2")]
    #[case(Rank::Three, "3")]
    #[case(Rank::Four, "4")]
    #[case(Rank::Five, "5")]
    #[case(Rank::Six, "6")]
    #[case(Rank::Seven, "7")]
    #[case(Rank::Eight, "8")]
    #[case(Rank::Nine, "9")]
    #[case(Rank::Ten, "10")]
    #[case(Rank::Jack, "J")]
    #[case(Rank::Queen, "Q")]
    #[case(Rank::King, "K")]
    #[case(Rank::Ace, "A")]
    fn can_be_displayed(#[case] rank: Rank, #[case] expected_string: String) {
        assert_eq!(expected_string, format!("{}", rank))
    }
}
