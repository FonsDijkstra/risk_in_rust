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
    fn has_country(&self, country: CountryId) -> bool;
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

    pub fn setup(&mut self, countries: &HashSet<CountryId>, armies: u8) {
        self.countries = countries.clone();
    }
}

impl Player for HumanPlayer {
    fn has_country(&self, country: CountryId) -> bool {
        self.countries.contains(&country)
    }
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
    fn has_country(&self, country: CountryId) -> bool {
        self.countries.contains(&country)
    }
}

