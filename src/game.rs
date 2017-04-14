extern crate std;
extern crate rand;

use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use self::rand::Rng;
pub use board::{Map, Deck, CountryId};
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
        let armies = self.initial_number_of_armies();
        self.devide_countries(armies);
    }

    fn initial_number_of_armies(&self) -> u8 {
        match self.players.len() {
            4 => 30,
            5 => 25,
            6 => 20,
            _ => panic!("not enough players"),
        }
    }

    fn devide_countries(&mut self, armies: u8) {
        let mut countries = self.map.all_countries().collect::<Vec<_>>();
        rand::thread_rng().shuffle(&mut countries);

        let mut division = HashMap::new();
        for (i, country) in countries.iter().enumerate() {
            division.entry(i % self.players.len()).or_insert(HashSet::new()).insert(*country);
        }

        for (index, countries) in division.iter() {
            self.players[*index].setup(countries, armies - countries.len() as u8);
        }
    }
}

