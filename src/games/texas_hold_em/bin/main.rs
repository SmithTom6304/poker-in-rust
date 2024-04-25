use std::fs;

use poker_in_rust::{
    card::Card,
    games::texas_hold_em::evaluation::{evaluator::Evaluator, hand_rank::HandRank},
    rank::Rank,
    suit::Suit,
};

fn main() {
    let rank_db = fs::read("res/HandRanks.dat").expect("Could not load hand ranks db");
    let rank_db = Evaluator::try_from(rank_db).expect("Could not parse hand ranks db");

    let pair = [
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Ace),
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::Jack),
        Card::new(Suit::Spade, Rank::Three),
    ];

    let two_pair = [
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Club, Rank::Ace),
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::Jack),
        Card::new(Suit::Spade, Rank::Ace),
    ];

    println!(
        "Pair = {:?}",
        HandRank::try_from(rank_db.evaluate_hand(&pair))
    );
    println!(
        "Two Pair = {:?}",
        HandRank::try_from(rank_db.evaluate_hand(&two_pair))
    );
}
