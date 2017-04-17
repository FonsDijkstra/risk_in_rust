extern crate std;
extern crate rand;

use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use self::rand::Rng;
pub use board::{Map, Deck, CountryId};
pub use player::{Player, DummyPlayer, HumanPlayer, Color};

#[derive(Debug)]
pub struct Game {
    map: Map,
    deck: Deck,
    dummy: DummyPlayer,
    players: Vec<HumanPlayer>,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let deck = Deck::new(&map);
        Game { 
            map: map,
            deck: deck,
            dummy: DummyPlayer::new(),
            players: Vec::with_capacity(6),
        }
    }

    pub fn add_player(&mut self, color: Color) {
        self.players.push(HumanPlayer::new(color))
    }

    pub fn start(&mut self) {
        let armies = self.initial_number_of_armies();
        self.setup_countries(armies);
        self.setup_turn();
    }

    fn initial_number_of_armies(&self) -> u8 {
        match self.players.len() {
            3 => 35,
            4 => 30,
            5 => 25,
            6 => 20,
            _ => panic!("not enough players"),
        }
    }

    fn setup_countries(&mut self, armies: u8) {
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

    fn setup_turn(&mut self) {
        let country = self.first_country_from_deck();
        let start = self.players
            .iter()
            .position(|ref player| player.has_country(country))
            .expect("all countries are divided");
        let mut players_turn = self.players.split_off(start);
        players_turn.append(&mut self.players);
        players_turn.shrink_to_fit();
        self.players = players_turn;
    }

    fn first_country_from_deck(&mut self) -> CountryId {
        loop {
            let card = self.deck.take_card();
            match card.country() {
                None => self.deck.return_card(card),
                Some(country) => {
                    self.deck.return_card(card);
                    return country;
                }
            }
        }
    }
}

