extern crate std;
extern crate rand;

use std::vec::Vec;
use self::rand::Rng;
pub use board::{Map, Deck, Strength};
pub use player::{Player, Color};

#[derive(Debug)]
pub struct Game {
    map: Map,
    deck: Deck,
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        let map = Map::new();
        let deck = Deck::new(&map);
        Game { map: map, deck: deck, players: Vec::new() }
    }

    pub fn add_player(&mut self, color: Color) {
        self.players.push(Player::new(color))
    }

    pub fn start(&mut self) {
        self.devide_countries()
    }

    fn devide_countries(&mut self) {
        let mut countries = self.map.all_countries().collect::<Vec<_>>();
        rand::thread_rng().shuffle(&mut countries);

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

