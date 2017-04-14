extern crate std;
extern crate rand;

use std::vec::Vec;
use self::rand::Rng;
pub use board::{Map, Deck, Strength};
pub use player::{DummyPlayer, HumanPlayer, Color};

#[derive(Debug)]
pub struct Game {
    map: Map,
    deck: Deck,
    dummy: DummyPlayer,
    players: Vec<HumanPlayer>,
    turn: usize,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let deck = Deck::new(&map);
        Game { 
            map: map,
            deck: deck,
            dummy: DummyPlayer::new(),
            players: Vec::with_capacity(7),
            turn: 0
        }
    }

    pub fn add_player(&mut self, color: Color) {
        self.players.push(HumanPlayer::new(color))
    }

    pub fn start(&mut self) {
        self.devide_countries();
    }

    fn devide_countries(&mut self) {
        let mut countries = self.map.all_countries().collect::<Vec<_>>();
        rand::thread_rng().shuffle(&mut countries);

        // we cannot use self.players.iter_mut().cycle()
        let mut index = 0usize;
        for country in countries {
            if index == self.players.len() {
                index = 0usize;
            }
            self.players.get_mut(index).expect("player should exist").add_country(country);
            index += 1
        }
    }
}

