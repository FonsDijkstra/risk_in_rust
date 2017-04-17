extern crate std;
extern crate rand;

use std::fmt;
use std::vec::Vec;
use std::boxed::Box;
use self::rand::Rng;
use board::country::{CountryId, Map};

#[derive(Debug, Copy, Clone)]
pub enum Strength {
    Artillery,
    Infantry,
    Cavelry,
    Wild
}

pub trait Card : fmt::Debug {
    fn country(&self) -> Option<CountryId>;
    fn strength(&self) -> Strength;

    fn armies(&self) -> u8 {
        match self.strength() {
            Strength::Artillery => 4,
            Strength::Infantry => 6,
            Strength::Cavelry => 8,
            Strength::Wild => 5,
        }
    }
}

#[derive(Debug)]
pub struct CountryCard {
    country: CountryId,
    strength: Strength,
}

impl CountryCard {
    pub fn new(country: CountryId, strength: Strength) -> Self {
        CountryCard { country: country, strength: strength }
    }
}

impl Card for CountryCard {
    fn country(&self) -> Option<CountryId> {
        Some(self.country)
    }

    fn strength(&self) -> Strength {
        self.strength
    }
}

#[derive(Debug)]
pub struct WildCard;

impl Card for WildCard {
    fn country(&self) -> Option<CountryId> {
        None
    }

    fn strength(&self) -> Strength {
        Strength::Wild
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Box<Card>>,
}

impl Deck {
    pub fn new(map: &Map) -> Deck {
        let mut cards = Vec::<Box<Card>>::new();
        cards.push(Box::new(WildCard));
        cards.push(Box::new(WildCard));
        for card in map.all_countries().map(|id| CountryCard::new(id, map.get_country(id).get_strength())) {
            cards.push(Box::new(card));
        }
        let mut deck = Deck { cards: cards };
        deck.shuffle();
        deck
    }

    pub fn take_card(&mut self) -> Box<Card> {
        self.cards.pop().expect("deck is empty")
    }

    pub fn return_card(&mut self, card: Box<Card>) {
        self.cards.push(card);
        self.shuffle()
    }

    fn shuffle(&mut self) {
        rand::thread_rng().shuffle(&mut self.cards)
    }
}

