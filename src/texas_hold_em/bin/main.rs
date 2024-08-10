use std::env;

use poker_in_rust::{
    player::{Folded, Player, PlayerId},
    texas_hold_em::{
        evaluation::two_plus_two_evaluator::TwoPlusTwoEvaluator,
        state::{
            advancement::Advancement,
            stages::{finished::Finished, pre_round::PreRound},
        },
    },
    Deck, Hand,
};

fn main() {
    let exe_path = env::current_exe().expect("Could not find current exe");
    let path = exe_path.parent().unwrap().join("HandRanks.dat");
    let evaluator =
        TwoPlusTwoEvaluator::create_from_path(&path).expect("Could not parse hand ranks db");

    // let game = Game::new(3, Box::new(evaluator));
    // game.do_round()

    let mut deck = Deck::new().shuffle();

    let players = vec![
        deal_player(PlayerId(1), &mut deck),
        deal_player(PlayerId(2), &mut deck),
        deal_player(PlayerId(3), &mut deck),
    ];
    let stage = PreRound::new(players).unwrap();
    let stage = stage.start_round();

    let stage = match stage.advance() {
        Advancement::NextStage(stage) => stage,
        Advancement::Finished(finished) => {
            finish_game(finished);
            return;
        }
    };

    let stage = match stage.advance() {
        Advancement::NextStage(stage) => stage,
        Advancement::Finished(finished) => {
            finish_game(finished);
            return;
        }
    };

    let stage = match stage.advance() {
        Advancement::NextStage(stage) => stage,
        Advancement::Finished(finished) => {
            finish_game(finished);
            return;
        }
    };

    let stage = match stage.advance() {
        Advancement::NextStage(stage) => stage,
        Advancement::Finished(finished) => {
            finish_game(finished);
            return;
        }
    };

    let finished = stage.finish(Box::new(evaluator));
    finish_game(finished);
}

fn finish_game(finished: Finished) -> PreRound {
    finished.payout()
}

fn deal_player(id: PlayerId, deck: &mut Deck) -> Player<Folded> {
    let cards = [deck.draw().unwrap(), deck.draw().unwrap()];
    Player::<Folded>::new(id, Hand::new(cards), 100)
}
