use crate::Card;

pub trait Evaluator {
    fn evaluate_hand(&self, cards: &[Card; 7]) -> HandVal;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandVal(pub u32);
