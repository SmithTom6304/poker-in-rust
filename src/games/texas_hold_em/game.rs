use std::collections::HashMap;

use crate::{
    card::Card,
    deck::Deck,
    games::texas_hold_em::evaluation::evaluator::HandVal,
    hand::Hand,
    player::{FoldedPlayer, Player, PlayerId},
    pot::Pot,
};

use super::{
    console_player::ConsolePlayer,
    evaluation::evaluator::Evaluator,
    player_driver::{Move, PlayerDriver},
};

pub struct Game {
    pub players: Vec<Player>,
    pub folded_players: Vec<FoldedPlayer>,
    pub deck: Deck,
    pub button_index: usize,
    pub current_player_index: usize,
    pub pot: Pot,
    pub flop: Option<[Card; 3]>,
    pub turn: Option<Card>,
    pub river: Option<Card>,
    evaluator: Evaluator,
}

impl Game {
    pub fn new(num_players: u8, evaluator: Evaluator) -> Self {
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
            flop: None,
            turn: None,
            river: None,
            evaluator,
        }
    }

    pub fn run(self) {
        let mut game = self;
        let mut game_finished = false;
        game.pot.minimum_bet = 10;

        loop {
            if game_finished {
                break;
            }

            Self::print_pre_round_status(&game);
            Self::do_round(&mut game);
            game.pot.minimum_bet = 0;

            if game.players.len() == 1 {
                game_finished = true;
                continue;
            }

            if game.flop.is_none() {
                game.flop = Some([
                    game.deck.draw().unwrap(),
                    game.deck.draw().unwrap(),
                    game.deck.draw().unwrap(),
                ]);
                continue;
            }

            if game.turn.is_none() {
                game.turn = Some(game.deck.draw().unwrap());
                continue;
            }

            if game.river.is_none() {
                game.river = Some(game.deck.draw().unwrap());
                continue;
            }

            game_finished = true;
        }

        if game.players.len() == 1 {
            game.pot = game.pot.deal_winnings(game.players.iter_mut().collect());
        } else {
            let mut cards = vec![];
            cards.append(&mut game.flop.unwrap().to_vec());
            cards.push(game.turn.unwrap());
            cards.push(game.river.unwrap());
            let mut player_score = HashMap::<PlayerId, HandVal>::new();
            for player in game.players.iter() {
                let mut cards = cards.clone();
                cards.append(&mut player.hand.cards.to_vec());
                let score = game.evaluator.evaluate_hand(&cards.try_into().unwrap());
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

        println!("Game done!");
        for player in game.players {
            println!("Player {:?} has {} chips", player, player.chips);
        }
        for player in game.folded_players {
            println!("Player {:?} has {} chips", player, player.chips);
        }
    }

    fn do_round(game: &mut Game) {
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
            let player_move = Self::determine_move(&game);
            game.handle_move(player_move);

            if game.current_player_index == game.button_index {
                game.advance_player();
                break;
            }

            game.advance_player();

            println!("Pot: {}, min bet: {}", game.pot.chips, game.pot.minimum_bet);
        }
    }

    fn print_pre_round_status(&self) {
        let mut turn_string = "Pre-flop";
        let mut cards = vec![];
        if let Some(flop) = self.flop {
            cards.append(&mut flop.to_vec());
            turn_string = "Flop";
        }
        if let Some(turn) = self.turn {
            cards.push(turn);
            turn_string = "Turn";
        }
        if let Some(river) = self.river {
            cards.push(river);
            turn_string = "River";
        }

        println!("Turn - {}", turn_string);
        println!(
            "Remaining players - {}",
            self.players
                .iter()
                .map(|player| format!("{}", player.id))
                .collect::<Vec<String>>()
                .join(", ")
        );
        if cards.len() > 0 {
            println!(
                "Cards - {}",
                cards
                    .iter()
                    .map(|card| format!("{}", card))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
    }

    fn determine_move(&self) -> Move {
        let player_move = ConsolePlayer::determine_move(&self);
        println!(
            "Player {:?} chose {:?}",
            self.players[self.current_player_index].id, player_move
        );

        player_move
    }

    fn handle_move(&mut self, player_move: Move) {
        match player_move {
            Move::Fold => self.handle_fold(),
            Move::Call => self.handle_call(),
            Move::Raise { amount } => self.handle_raise(amount),
        }
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
