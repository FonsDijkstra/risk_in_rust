extern crate std;

use std::collections::HashSet;
pub use board::{CountryId, Country};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Blue,
    Yellow,
    Pink,
    Green,
}

#[derive(Debug)]
pub struct Player {
    color: Color,
    countries: HashSet<CountryId>
}

impl Player {
    pub fn new(color: Color) -> Player {
        Player { color: color, countries: HashSet::new() }
    }

    pub fn add_country(&mut self, country: CountryId) {
        if !self.countries.insert(country) {
            panic!("Player already has country");
        }
    }
}

