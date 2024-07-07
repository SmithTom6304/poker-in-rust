use crate::{
    games::texas_hold_em::{
        console_player::ConsolePlayer,
        player_driver::{Move, PlayerDriver},
    },
    player::{Active, Folded, Player},
    pot::Pot,
};

#[derive(Debug)]
pub struct GameLoop {
    pub active_players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub button_index: usize,
    pub current_player_index: usize,
    pub pot: Pot,
}

#[derive(Debug)]
pub enum StageOutcome {
    NextStage(GameLoop),
    Finished(GameLoop),
}

#[derive(Debug, PartialEq)]
enum MoveOutcome {
    StageFinished,
    RoundFinished,
    NextMove,
}

impl GameLoop {
    pub fn do_stage(mut self) -> StageOutcome {
        let mut move_outcome = MoveOutcome::NextMove;
        while move_outcome == MoveOutcome::NextMove {
            let player_move = self.determine_move();
            move_outcome = self.do_move(player_move);
        }

        match move_outcome {
            MoveOutcome::StageFinished => StageOutcome::NextStage(self),
            MoveOutcome::RoundFinished => StageOutcome::Finished(self),
            MoveOutcome::NextMove => panic!(),
        }
    }

    fn determine_move(&self) -> Move {
        let player_move = ConsolePlayer::determine_move(self);
        println!(
            "Player {:?} chose {:?}",
            self.active_players[self.current_player_index].id, player_move
        );

        player_move
    }

    fn do_move(&mut self, player_move: Move) -> MoveOutcome {
        let mut advance_stage = false;
        match player_move {
            Move::Fold => {
                self.handle_fold();
                advance_stage = self.current_player_index == self.button_index;
            }
            Move::Call => {
                self.handle_call();
                advance_stage = self.current_player_index == self.button_index;
            }
            Move::Raise { amount } => self.handle_raise(amount),
        }

        if self.active_players.len() == 1 {
            return MoveOutcome::RoundFinished;
        }

        self.advance_player(player_move);

        if advance_stage {
            return MoveOutcome::StageFinished;
        }

        MoveOutcome::NextMove
    }

    fn handle_fold(&mut self) {
        let current_player = self.active_players.remove(self.current_player_index);
        let folded_player = current_player.fold();
        self.folded_players.push(folded_player);
    }

    fn handle_call(&mut self) {
        let current_player = &mut self.active_players[self.current_player_index];
        match current_player.bet(self.pot.minimum_bet, &mut self.pot) {
            Ok(_) => (),
            Err(err) => {
                println!("Error performing player call: {}. Folding instead", err);
                self.handle_fold();
            }
        }
    }

    fn handle_raise(&mut self, amount: u32) {
        let current_player = &mut self.active_players[self.current_player_index];
        match current_player.bet(amount, &mut self.pot) {
            Ok(_) => println!("Minimum bet: {}", amount),
            Err(err) => {
                println!("Error performing player raise: {}. Folding instead", err);
                self.handle_fold();
            }
        }
    }

    fn advance_player(&mut self, player_move: Move) {
        match player_move {
            Move::Fold => {
                if self.current_player_index <= self.button_index {
                    self.button_index = match self.button_index {
                        0 => self.active_players.len() - 1,
                        _ => self.button_index - 1,
                    }
                }

                if self.current_player_index >= self.active_players.len() {
                    self.current_player_index = 0
                }
            }
            Move::Call => {
                self.current_player_index =
                    (self.current_player_index + 1) % self.active_players.len()
            }
            Move::Raise { amount } => {
                self.handle_raise(amount);
                self.button_index = match self.current_player_index {
                    0 => self.active_players.len() - 1,
                    _ => self.current_player_index - 1,
                };
                self.current_player_index =
                    (self.current_player_index + 1) % self.active_players.len()
            }
        }
    }
}
