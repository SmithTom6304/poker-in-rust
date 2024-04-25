use std::path::Path;

use crate::{card::Card, deck::Deck};

use super::{database::DataBase, hand_rank::HandRank};

const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

const SUIT_BITS: [u32; 4] = [8, 4, 2, 1];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandVal(u32);

#[derive(Debug, PartialEq)]
struct CardNum(u32);
#[derive(Debug, PartialEq)]
struct CardCode(u32);

pub struct Evaluator {
    data: DataBase,
    deck: Vec<CardCode>,
}

impl Evaluator {
    pub fn create_from_path(path: &Path) -> Result<Self, String> {
        let data = match DataBase::load_from_path(path) {
            Ok(database) => database,
            Err(err) => return Err(err),
        };

        let deck = Deck::new()
            .cards
            .iter()
            .map(|card| CardCode::from(card))
            .collect();

        Ok(Evaluator { data, deck })
    }

    pub fn evaluate_hand(&self, cards: &[Card; 7]) -> HandVal {
        let mut result = 53;
        let card_nums = cards.iter().map(|card| Evaluator::card_to_num(&self, card));
        for num in card_nums {
            result = self.data.0[(result + num.0) as usize];
        }
        HandVal(result)
    }

    fn card_to_num(&self, card: &Card) -> CardNum {
        let card_code = CardCode::from(card);
        let result = self.deck.iter().position(|code| code == &card_code);
        CardNum(result.expect("Could not find card code in deck") as u32 + 1) // Array starts at one for some reason..
    }
}

impl From<&Card> for CardCode {
    fn from(value: &Card) -> Self {
        let ri = value.rank as u32;
        CardCode(
            PRIMES[value.rank as usize]
                | (ri << 8)
                | SUIT_BITS[value.suit as usize] << 12
                | (1 << (16 + ri)),
        )
    }
}

impl TryFrom<HandVal> for HandRank {
    type Error = String;

    fn try_from(value: HandVal) -> Result<Self, Self::Error> {
        let result = value.0 >> 12;
        match result {
            1 => Ok(HandRank::HighCard),
            2 => Ok(HandRank::Pair),
            3 => Ok(HandRank::TwoPair),
            4 => Ok(HandRank::ThreeOfAKind),
            5 => Ok(HandRank::Straight),
            6 => Ok(HandRank::Flush),
            7 => Ok(HandRank::FullHouse),
            8 => Ok(HandRank::FourOfAKind),
            9 => Ok(HandRank::StraightFlush),
            _ => Err(format!("HandVal {} not recognized", result)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{rank::Rank, suit::Suit};

    use super::*;
    use rstest::rstest;

    fn create_evaluator() -> Evaluator {
        let path = PathBuf::from("two-plus-two-hand-evaluator/HandRanks.dat");
        Evaluator::create_from_path(&path).expect("Could not parse hand ranks db")
    }

    #[rstest]
    #[case(Card::new(Suit::Club, Rank::Two), CardNum(1))]
    #[case(Card::new(Suit::Diamond, Rank::Two), CardNum(2))]
    #[case(Card::new(Suit::Heart, Rank::Two), CardNum(3))]
    #[case(Card::new(Suit::Spade, Rank::Two), CardNum(4))]
    fn card_has_correct_card_number(#[case] card: Card, #[case] expected_card_number: CardNum) {
        let evaluator = create_evaluator();
        assert_eq!(expected_card_number, evaluator.card_to_num(&card));
    }

    #[rstest]
    #[case(Card::new(Suit::Diamond, Rank::King), CardCode(134236965))]
    #[case(Card::new(Suit::Spade, Rank::Five), CardCode(529159))]
    #[case(Card::new(Suit::Club, Rank::Jack), CardCode(33589533))]
    fn card_has_correct_code(#[case] card: Card, #[case] expected_card_code: CardCode) {
        assert_eq!(expected_card_code, CardCode::from(&card));
    }

    #[rstest]
    #[case([
            Card::new(Suit::Club, Rank::Two),
            Card::new(Suit::Diamond, Rank::Two),
            Card::new(Suit::Heart, Rank::Two),
            Card::new(Suit::Spade, Rank::Two),
            Card::new(Suit::Club, Rank::Three),
            Card::new(Suit::Diamond, Rank::Three),
            Card::new(Suit::Heart, Rank::Three),
        ], [106, 2862, 73140, 1244440, 5720184, 13808355, 32769])]
    #[case([
            Card::new(Suit::Club, Rank::Two),
            Card::new(Suit::Club, Rank::Three),
            Card::new(Suit::Club, Rank::Four),
            Card::new(Suit::Club, Rank::Five),
            Card::new(Suit::Club, Rank::Six),
            Card::new(Suit::Club, Rank::Seven),
            Card::new(Suit::Club, Rank::Eight),
        ], [106, 3021, 81090, 1581149, 6820464, 16594035, 36868])]
    fn eval_card_num_hand(#[case] cards: [Card; 7], #[case] expected_values: [u32; 7]) {
        let evaluator = create_evaluator();

        let cnums = cards.iter().map(|card| evaluator.card_to_num(card));

        let mut result = 53;
        let mut i = 0;
        for cnum in cnums {
            result = evaluator.data.0[(result + cnum.0) as usize];
            assert_eq!(result, expected_values[i]);
            i += 1;
        }
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Ace),
        Card::new(Suit::Spade, Rank::Queen),
        Card::new(Suit::Club, Rank::Two),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Spade, Rank::Ten),
        Card::new(Suit::Spade, Rank::Jack)
        ])]
    fn scores_royal_flush_as_straight_flush(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::StraightFlush,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Spade, Rank::Queen),
        Card::new(Suit::Club, Rank::Two),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Spade, Rank::Ten),
        Card::new(Suit::Spade, Rank::Jack)
        ])]
    fn can_score_straight_flush(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::StraightFlush,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Diamond, Rank::Nine),
        Card::new(Suit::Heart, Rank::Nine),
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Ten),
        Card::new(Suit::Diamond, Rank::Jack),
        Card::new(Suit::Heart, Rank::King)
    ])]
    #[case([
        Card::new(Suit::Club, Rank::Two),
        Card::new(Suit::Diamond, Rank::Two),
        Card::new(Suit::Club, Rank::Three),
        Card::new(Suit::Heart, Rank::Two),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::Three),
        Card::new(Suit::Heart, Rank::Three),
    ])]
    fn can_score_four_of_a_kind(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::FourOfAKind,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Heart, Rank::Nine),
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Diamond, Rank::Ten),
        Card::new(Suit::Spade, Rank::Ten),
        Card::new(Suit::Spade, Rank::Jack)
        ])]
    fn can_score_full_house(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::FullHouse,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::Nine),
        Card::new(Suit::Club, Rank::Ten),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Club, Rank::Two),
        Card::new(Suit::Club, Rank::Five),
        Card::new(Suit::Club, Rank::Four)
        ])]
    fn can_score_flush(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::Flush,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Queen),
        Card::new(Suit::Club, Rank::Two),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Diamond, Rank::Ten),
        Card::new(Suit::Spade, Rank::Jack)
        ])]
    fn can_score_straight(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::Straight,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Queen),
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::Nine),
        Card::new(Suit::Spade, Rank::Ace)
        ])]
    fn can_score_three_of_a_kind(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::ThreeOfAKind,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Ace),
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::Jack),
        Card::new(Suit::Spade, Rank::Ace)
        ])]
    fn can_score_two_pair(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::TwoPair,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Ace),
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::Jack),
        Card::new(Suit::Spade, Rank::Three)
        ]
    )]
    fn can_score_pair(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::Pair,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }

    #[rstest]
    #[case([
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Ace),
        Card::new(Suit::Club, Rank::Queen),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::Jack),
        Card::new(Suit::Spade, Rank::Three)
        ]
    )]
    fn can_score_high_card(#[case] cards: [Card; 7]) {
        let evaluator = create_evaluator();
        assert_eq!(
            HandRank::HighCard,
            HandRank::try_from(evaluator.evaluate_hand(&cards)).unwrap()
        )
    }
}
