extern crate petgraph;

use board::card::Strength;
use self::petgraph::graph::{UnGraph, NodeIndex, NodeIndices};

pub type CountryId = NodeIndex;
type CountryIterator = NodeIndices;

#[derive(Debug)]
pub struct Country {
    name: &'static str,
    strength: Strength,
    armies: u8,
}

impl Country {
    fn new(name: &'static str, strength: Strength) -> Self {
        Country { name: name, strength: strength, armies: 1 }
    }

    pub fn get_strength(&self) -> Strength {
        self.strength
    }
}

#[derive(Debug)]
pub struct Map {
    graph: UnGraph<Country, ()>,
}

impl Map {
    pub fn new() -> Self {
        let mut graph = UnGraph::<_, ()>::new_undirected();

        let venezuela = graph.add_node(Country::new("Venezuela", Strength::Artillery)); 
        let brazil = graph.add_node(Country::new("Brazil", Strength::Artillery));
        let peru = graph.add_node(Country::new("Peru", Strength::Cavelry));
        let argentina = graph.add_node(Country::new("Argentine", Strength::Infantry));

        let egypt = graph.add_node(Country::new("Egypt", Strength::Infantry));
        let east_africa = graph.add_node(Country::new("East Africa", Strength::Artillery));
        let north_africa = graph.add_node(Country::new("North Africa", Strength::Infantry));
        let south_africa = graph.add_node(Country::new("South Africa", Strength::Artillery));
        let central_africa = graph.add_node(Country::new("Central Africa", Strength::Cavelry));
        let madagascar = graph.add_node(Country::new("Madagascar", Strength::Infantry));

        graph.add_edge(venezuela, peru, ());
        graph.add_edge(venezuela, brazil, ());
        graph.add_edge(peru, brazil, ());
        graph.add_edge(peru, argentina, ());
        graph.add_edge(brazil, argentina, ());

        graph.add_edge(north_africa, brazil, ());
        graph.add_edge(north_africa, egypt, ());
        graph.add_edge(north_africa, east_africa, ());
        graph.add_edge(north_africa, central_africa, ());
        graph.add_edge(egypt, east_africa, ());
        graph.add_edge(east_africa, central_africa, ());
        graph.add_edge(east_africa, madagascar, ());
        graph.add_edge(east_africa, south_africa, ());
        graph.add_edge(central_africa, south_africa, ());
        graph.add_edge(madagascar, south_africa, ());

        Map { graph: graph }
    }

    pub fn all_countries(&self) ->  CountryIterator {
        self.graph.node_indices()
    }

    pub fn get_country(&self, id: CountryId) -> &Country {
        self.graph.node_weight(id).expect("unknown country id")
    }
}

