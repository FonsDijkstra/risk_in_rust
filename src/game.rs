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
        self.players.push(Player::new(color));
    }

    pub fn start(&mut self) {
        self.devide_countries()
    }

    fn devide_countries(&mut self) {
        let mut rng = rand::thread_rng();
        let mut countries = self.map.all_countries().collect::<Vec<_>>();
        rng.shuffle(&mut countries);

        for player in self.players.iter().cycle() {
            match countries.pop() {
                Some(country) => println!("{:?}: {:?}", player, country),
                None => break,
            }
        }
    }
}

