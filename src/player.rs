extern crate std;

use std::collections::HashMap;
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
    countries: HashMap<CountryId, ()>,
}

impl Player {
    pub fn new(color: Color) -> Player {
        Player { color: color, countries: HashMap::new() }
    }
}

