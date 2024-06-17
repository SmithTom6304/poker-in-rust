use core::fmt;
use std::collections::HashMap;

use crate::{
    card::Card,
    deck::Deck,
    games::texas_hold_em::evaluation::evaluator::{Evaluator, HandVal},
    hand::Hand,
    player::{Active, Folded, Player, PlayerId},
    pot::Pot,
};

use super::{
    console_player::ConsolePlayer,
    player_driver::{Move, PlayerDriver},
};

pub struct Game {
    pub players: Vec<Player<Active>>,
    pub folded_players: Vec<Player<Folded>>,
    pub deck: Deck,
    pub button_index: usize,
    pub current_player_index: usize,
    pub pot: Pot,
    pub stage: Stage,
    evaluator: Box<dyn Evaluator>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Stage {
    PreFlop,
    Flop { cards: [Card; 3] },
    Turn { cards: [Card; 4] },
    River { cards: [Card; 5] },
    Finished,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cards_vec = vec![];
        match self {
            Stage::PreFlop => {
                write!(f, "Pre-Flop")?;
            }
            Stage::Flop { cards } => {
                write!(f, "Flop")?;
                cards_vec.append(&mut cards.to_vec());
            }
            Stage::Turn { cards } => {
                write!(f, "Turn")?;
                cards_vec.append(&mut cards.to_vec());
            }
            Stage::River { cards } => {
                write!(f, "River")?;
                cards_vec.append(&mut cards.to_vec());
            }
            Stage::Finished => {
                write!(f, "Finished")?;
            }
        }

        if !cards_vec.is_empty() {
            write!(
                f,
                "Cards - {}",
                cards_vec
                    .iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }

        Ok(())
    }
}

impl Game {
    pub fn new(num_players: u8, evaluator: Box<dyn Evaluator>) -> Self {
        let mut deck = Deck::new().shuffle();
        let button = num_players - 1;
        let current_player = 0;
        let pot = Pot::empty();
        let mut players = vec![];
        let mut i = 1;
        while i <= num_players {
            players.push(Player::<Active>::new(
                PlayerId(i),
                Hand::new([deck.draw().unwrap(), deck.draw().unwrap()]),
                100,
            ));
            i += 1;
        }

        Game {
            players,
            folded_players: vec![],
            deck,
            button_index: button as usize,
            current_player_index: current_player,
            pot,
            stage: Stage::PreFlop,
            evaluator,
        }
    }

    pub fn do_round(self) {
        let mut game = self;
        game.pot.minimum_bet = 10;

        loop {
            Self::print_pre_stage_status(&game);
            Self::do_stage(&mut game);
            game.pot.minimum_bet = 0;

            if game.stage == Stage::Finished {
                break;
            }
        }

        println!("Game done!");
        for player in game.players {
            println!("Player {:?} has {} chips", player, player.chips);
        }
        for player in game.folded_players {
            println!("Player {:?} has {} chips", player, player.chips);
        }
    }

    pub fn do_stage(game: &mut Game) {
        loop {
            let stage = game.stage;
            println!(
                "Player {:?}'s turn",
                game.players[game.current_player_index].id
            );
            println!(
                "Player {:?}'s cards: {}",
                game.players[game.current_player_index].id,
                game.players[game.current_player_index]
                    .hand
                    .cards
                    .iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            let player_move = Self::determine_move(game);
            game.do_move(player_move);

            println!("Pot: {}, min bet: {}", game.pot.chips, game.pot.minimum_bet);

            if stage != game.stage {
                break;
            }
        }
    }

    pub fn do_move(&mut self, player_move: Move) {
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

        if self.players.len() == 1 {
            self.finish_round();
            self.stage = Stage::Finished;
            return;
        }

        self.advance_player(player_move);

        if advance_stage {
            self.advance_stage()
        }
    }

    fn advance_player(&mut self, player_move: Move) {
        match player_move {
            Move::Fold => {
                if self.current_player_index <= self.button_index {
                    self.button_index = match self.button_index {
                        0 => self.players.len() - 1,
                        _ => self.button_index - 1,
                    }
                }

                if self.current_player_index >= self.players.len() {
                    self.current_player_index = 0
                }
            }
            Move::Call => {
                self.current_player_index = (self.current_player_index + 1) % self.players.len()
            }
            Move::Raise { amount } => {
                self.handle_raise(amount);
                self.button_index = match self.current_player_index {
                    0 => self.players.len() - 1,
                    _ => self.current_player_index - 1,
                };
                self.current_player_index = (self.current_player_index + 1) % self.players.len()
            }
        }
    }

    fn advance_stage(&mut self) {
        self.stage = match self.stage {
            Stage::PreFlop => Stage::Flop {
                cards: Self::draw_flop(&mut self.deck),
            },
            Stage::Flop { cards } => Stage::Turn {
                cards: Self::draw_turn(&mut self.deck, cards),
            },
            Stage::Turn { cards } => Stage::River {
                cards: Self::draw_river(&mut self.deck, cards),
            },
            Stage::River { cards: _ } => {
                self.finish_round();
                Stage::Finished
            }
            Stage::Finished => {
                //Error?
                return;
            }
        };
    }

    fn finish_round(&mut self) {
        if self.players.len() == 1 {
            let players = self.players.iter_mut().collect();
            self.pot.deal_winnings(players);
        } else {
            let cards = match self.stage {
                Stage::River { cards } => cards,
                _ => panic!("Incorrect game stage"),
            };

            let mut player_score = HashMap::<PlayerId, HandVal>::new();
            for player in self.players.iter() {
                let cards = cards;
                let cards = [
                    cards[0],
                    cards[1],
                    cards[2],
                    cards[3],
                    cards[4],
                    player.hand.cards[0],
                    player.hand.cards[1],
                ];

                let score = self.evaluator.evaluate_hand(&cards);
                player_score.insert(player.id, score);
            }

            let winner_score = player_score.values().max().unwrap();
            let winners = player_score
                .iter()
                .filter(|pair| pair.1 == winner_score)
                .map(|pair| pair.0)
                .collect::<Vec<&PlayerId>>();
            let winners = self
                .players
                .iter_mut()
                .filter(|player| winners.contains(&&player.id))
                .collect::<Vec<&mut Player<Active>>>();
            println!(
                "Winners: {}",
                winners
                    .iter()
                    .map(|player| format!("{}", player.id))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            self.pot.deal_winnings(winners);
        }
    }

    fn print_pre_stage_status(&self) {
        println!("{}", self.stage);
        println!(
            "Remaining players - {}",
            self.players
                .iter()
                .map(|player| format!("{}", player.id))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    fn draw_flop(deck: &mut Deck) -> [Card; 3] {
        [
            deck.draw().unwrap(),
            deck.draw().unwrap(),
            deck.draw().unwrap(),
        ]
    }

    fn draw_turn(deck: &mut Deck, flop: [Card; 3]) -> [Card; 4] {
        [flop[0], flop[1], flop[2], deck.draw().unwrap()]
    }

    fn draw_river(deck: &mut Deck, turn: [Card; 4]) -> [Card; 5] {
        [turn[0], turn[1], turn[2], turn[3], deck.draw().unwrap()]
    }

    fn determine_move(&self) -> Move {
        let player_move = ConsolePlayer::determine_move(self);
        println!(
            "Player {:?} chose {:?}",
            self.players[self.current_player_index].id, player_move
        );

        player_move
    }

    fn handle_fold(&mut self) {
        let current_player = self.players.remove(self.current_player_index);
        let folded_player = current_player.fold();
        self.folded_players.push(folded_player);
    }

    fn handle_call(&mut self) {
        let current_player = &mut self.players[self.current_player_index];
        match current_player.bet(self.pot.minimum_bet, &mut self.pot) {
            Ok(_) => (),
            Err(err) => {
                println!("Error performing player call: {}. Folding instead", err);
                self.handle_fold();
            }
        }
    }

    fn handle_raise(&mut self, amount: u32) {
        let current_player = &mut self.players[self.current_player_index];
        match current_player.bet(amount, &mut self.pot) {
            Ok(_) => println!("Minimum bet: {}", amount),
            Err(err) => {
                println!("Error performing player raise: {}. Folding instead", err);
                self.handle_fold();
            }
        }
    }
}
