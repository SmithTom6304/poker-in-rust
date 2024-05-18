use std::env;

use poker_in_rust::games::texas_hold_em::{evaluation::evaluator::Evaluator, game::Game};

fn main() {
    let exe_path = env::current_exe().expect("Could not find current exe");
    let path = exe_path.parent().unwrap().join("HandRanks.dat");
    let evaluator = Evaluator::create_from_path(&path).expect("Could not parse hand ranks db");

    let game = Game::new(3, evaluator);
    game.do_round()
}
