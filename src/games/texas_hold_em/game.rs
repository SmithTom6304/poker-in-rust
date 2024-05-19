use std::collections::HashMap;

use crate::{
    card::Card,
    deck::Deck,
    games::texas_hold_em::evaluation::evaluator::{Evaluator, HandVal},
    hand::Hand,
    player::{FoldedPlayer, Player, PlayerId},
    pot::Pot,
};

use super::{
    console_player::ConsolePlayer,
    player_driver::{Move, PlayerDriver},
};

pub struct Game {
    pub players: Vec<Player>,
    pub folded_players: Vec<FoldedPlayer>,
    pub deck: Deck,
    pub button_index: usize,
    pub current_player_index: usize,
    pub pot: Pot,
    stage: Stage,
    evaluator: Box<dyn Evaluator>,
}

pub enum Stage {
    PreFlop,
    Flop { cards: [Card; 3] },
    Turn { cards: [Card; 4] },
    River { cards: [Card; 5] },
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
            players.push(Player {
                id: PlayerId(i),
                hand: Hand::new([deck.draw().unwrap(), deck.draw().unwrap()]),
                chips: 100,
            });
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
        let mut game_finished = false;
        game.pot.minimum_bet = 10;

        loop {
            if game_finished {
                break;
            }

            Self::print_pre_stage_status(&game);
            Self::do_stage(&mut game);
            game.pot.minimum_bet = 0;

            if game.players.len() == 1 {
                game_finished = true;
                continue;
            }

            let stage = match game.stage {
                Stage::PreFlop => Stage::Flop {
                    cards: Self::draw_flop(&mut game.deck),
                },
                Stage::Flop { cards } => Stage::Turn {
                    cards: Self::draw_turn(&mut game.deck, cards),
                },
                Stage::Turn { cards } => Stage::River {
                    cards: Self::draw_river(&mut game.deck, cards),
                },
                Stage::River { cards: _ } => {
                    Self::finish_round(&mut game);
                    game_finished = true;
                    continue;
                }
            };
            game.stage = stage;
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
            if game.players.len() == 1 {
                break;
            }

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

            if game.current_player_index == game.button_index {
                game.advance_player();
                break;
            }

            game.advance_player();

            println!("Pot: {}, min bet: {}", game.pot.chips, game.pot.minimum_bet);
        }
    }

    pub fn do_move(&mut self, player_move: Move) {
        match player_move {
            Move::Fold => self.handle_fold(),
            Move::Call => self.handle_call(),
            Move::Raise { amount } => self.handle_raise(amount),
        }
    }

    fn finish_round(game: &mut Game) {
        if game.players.len() == 1 {
            let players = game.players.iter_mut().collect();
            game.pot.deal_winnings(players);
        } else {
            let cards = match game.stage {
                Stage::River { cards } => cards,
                _ => panic!("Incorrect game stage"),
            };

            let mut player_score = HashMap::<PlayerId, HandVal>::new();
            for player in game.players.iter() {
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

                let score = game.evaluator.evaluate_hand(&cards);
                player_score.insert(player.id, score);
            }

            let winner_score = player_score.values().max().unwrap();
            let winners = player_score
                .iter()
                .filter(|pair| pair.1 == winner_score)
                .map(|pair| pair.0)
                .collect::<Vec<&PlayerId>>();
            let winners = game
                .players
                .iter_mut()
                .filter(|player| winners.contains(&&player.id))
                .collect::<Vec<&mut Player>>();
            println!(
                "Winners: {}",
                winners
                    .iter()
                    .map(|player| format!("{}", player.id))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            game.pot.deal_winnings(winners);
        }
    }

    fn print_pre_stage_status(&self) {
        let mut cards_vec = vec![];
        match self.stage {
            Stage::PreFlop => println!("Pre-Flop"),
            Stage::Flop { cards } => {
                println!("Flop");
                cards_vec.append(&mut cards.to_vec());
            }
            Stage::Turn { cards } => {
                println!("Turn");
                cards_vec.append(&mut cards.to_vec());
            }
            Stage::River { cards } => {
                println!("River");
                cards_vec.append(&mut cards.to_vec());
            }
        }

        println!(
            "Remaining players - {}",
            self.players
                .iter()
                .map(|player| format!("{}", player.id))
                .collect::<Vec<String>>()
                .join(", ")
        );
        if !cards_vec.is_empty() {
            println!(
                "Cards - {}",
                cards_vec
                    .iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
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

        if self.current_player_index == 0 {
            self.current_player_index = self.players.len() - 1;
        } else {
            self.current_player_index -= 1;
        }
        if self.button_index == 0 {
            self.button_index = self.players.len() - 1;
        } else {
            self.button_index -= 1;
        }
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

        self.button_index = match self.current_player_index {
            0 => self.players.len() - 1,
            _ => self.current_player_index - 1,
        };
    }

    fn advance_player(&mut self) {
        self.current_player_index = (self.current_player_index + 1) % self.players.len();
    }
}
