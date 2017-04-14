extern crate std;

use std::fmt;
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

pub trait Player : fmt::Debug {
}

#[derive(Debug)]
pub struct HumanPlayer {
    color: Color,
    countries: HashSet<CountryId>
}

impl HumanPlayer {
    pub fn new(color: Color) -> Self {
        HumanPlayer { color: color, countries: HashSet::new() }
    }

    pub fn add_country(&mut self, country: CountryId) {
        if !self.countries.insert(country) {
            panic!("Player already has country");
        }
    }
}

impl Player for HumanPlayer {
}

#[derive(Debug)]
pub struct DummyPlayer {
    countries: HashSet<CountryId>
}

impl DummyPlayer {
    pub fn new() -> Self {
        DummyPlayer { countries: HashSet::new() }
    }
}

impl Player for DummyPlayer {
}

