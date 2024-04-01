use std::fmt;

#[derive(PartialEq, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_string = match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Ace => 'A',
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
    #[case(Rank::Ace, "A")]
    fn can_be_displayed(#[case] rank: Rank, #[case] expected_string: String) {
        assert_eq!(expected_string, format!("{}", rank))
    }
}
