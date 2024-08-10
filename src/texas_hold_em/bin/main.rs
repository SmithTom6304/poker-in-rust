use std::env;

use poker_in_rust::texas_hold_em::{
    evaluation::{evaluator::Evaluator, two_plus_two_evaluator::TwoPlusTwoEvaluator},
    state::{
        advancement::Advancement,
        stages::{finished::Finished, pre_round::PreRound},
    },
};

fn main() {
    let exe_path = env::current_exe().expect("Could not find current exe");
    let path = exe_path.parent().unwrap().join("HandRanks.dat");
    let evaluator =
        TwoPlusTwoEvaluator::create_from_path(&path).expect("Could not parse hand ranks db");
    let evaluator: Box<dyn Evaluator> = Box::new(evaluator);

    let mut pre_round = PreRound::new(3).unwrap();

    loop {
        let stage = pre_round.start_round();

        let stage = match stage.advance() {
            Advancement::NextStage(stage) => stage,
            Advancement::Finished(finished) => {
                pre_round = finish_game(finished);
                continue;
            }
        };

        let stage = match stage.advance() {
            Advancement::NextStage(stage) => stage,
            Advancement::Finished(finished) => {
                pre_round = finish_game(finished);
                continue;
            }
        };

        let stage = match stage.advance() {
            Advancement::NextStage(stage) => stage,
            Advancement::Finished(finished) => {
                pre_round = finish_game(finished);
                continue;
            }
        };

        let stage = match stage.advance() {
            Advancement::NextStage(stage) => stage,
            Advancement::Finished(finished) => {
                pre_round = finish_game(finished);
                continue;
            }
        };

        let finished = stage.finish(&evaluator);
        pre_round = finish_game(finished);
    }
}

fn finish_game(finished: Finished) -> PreRound {
    finished.payout()
}
