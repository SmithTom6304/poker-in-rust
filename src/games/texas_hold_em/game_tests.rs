use super::player_driver::Move;
use super::{
    evaluation::evaluator::{Evaluator, HandVal},
    game::Game,
};
use rstest::rstest;

pub struct DummyEvaluator {}
impl Evaluator for DummyEvaluator {
    fn evaluate_hand(
        &self,
        _cards: &[crate::card::Card; 7],
    ) -> super::evaluation::evaluator::HandVal {
        HandVal(0)
    }
}

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 0)]
fn folding_moves_current_player_index_correctly(
    #[case] player_to_fold: usize,
    #[case] expected_next_player: usize,
) {
    let mut game = Game::new(3, Box::new(DummyEvaluator {}));
    game.current_player_index = player_to_fold;

    game.do_move(Move::Fold);

    assert_eq!(expected_next_player, game.current_player_index)
}

#[rstest]
#[case(0, 0, 1)]
#[case(0, 1, 0)]
#[case(0, 2, 1)]
#[case(1, 0, 0)]
#[case(1, 1, 0)]
#[case(1, 2, 1)]
#[case(2, 0, 0)]
#[case(2, 1, 1)]
#[case(2, 2, 1)]
fn folding_moves_button_index_correctly(
    #[case] player_to_fold: usize,
    #[case] current_button: usize,
    #[case] expected_new_button: usize,
) {
    let mut game = Game::new(3, Box::new(DummyEvaluator {}));
    game.current_player_index = player_to_fold;
    game.button_index = current_button;

    game.do_move(Move::Fold);

    assert_eq!(expected_new_button, game.button_index)
}

#[rstest]
#[case(0, 1)]
#[case(1, 2)]
#[case(2, 0)]
fn calling_moves_current_player_index_correctly(
    #[case] player_to_call: usize,
    #[case] expected_next_player: usize,
) {
    let mut game = Game::new(3, Box::new(DummyEvaluator {}));
    game.current_player_index = player_to_call;

    game.do_move(Move::Call);

    assert_eq!(expected_next_player, game.current_player_index)
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn calling_does_not_move_button_index(#[case] player_to_call: usize) {
    let mut game = Game::new(3, Box::new(DummyEvaluator {}));
    let expected_button = game.button_index;
    game.current_player_index = player_to_call;

    game.do_move(Move::Call);

    assert_eq!(expected_button, game.button_index)
}

#[rstest]
#[case(0, 1)]
#[case(1, 2)]
#[case(2, 0)]
fn raising_moves_current_player_index_correctly(
    #[case] player_to_raise: usize,
    #[case] expected_next_player: usize,
) {
    let mut game = Game::new(3, Box::new(DummyEvaluator {}));
    game.current_player_index = player_to_raise;

    game.do_move(Move::Raise { amount: 10 });

    assert_eq!(expected_next_player, game.current_player_index)
}

#[rstest]
#[case(0, 0, 2)]
#[case(0, 1, 2)]
#[case(0, 2, 2)]
#[case(1, 0, 0)]
#[case(1, 1, 0)]
#[case(1, 2, 0)]
#[case(2, 0, 1)]
#[case(2, 1, 1)]
#[case(2, 2, 1)]
fn raising_moves_button_index_correctly(
    #[case] player_to_raise: usize,
    #[case] current_button: usize,
    #[case] expected_new_button: usize,
) {
    let mut game = Game::new(3, Box::new(DummyEvaluator {}));
    game.current_player_index = player_to_raise;
    game.button_index = current_button;

    game.do_move(Move::Raise { amount: 10 });

    assert_eq!(expected_new_button, game.button_index)
}
