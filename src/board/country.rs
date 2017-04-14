extern crate petgraph;

use board::card::Strength;
use self::petgraph::graph::{UnGraph, NodeIndex, NodeIndices};

pub type CountryId = NodeIndex;
type CountryIterator = NodeIndices;

#[derive(Debug)]
pub struct Country {
    name: &'static str,
    strength: Strength,
}

impl Country {
    pub fn get_strength(&self) -> Strength {
        self.strength
    }
}

#[derive(Debug)]
pub struct Map {
    graph: UnGraph<Country, ()>,
}

impl Map {
    pub fn new() -> Map {
        let mut graph = UnGraph::<_, ()>::new_undirected();

        let venezuela = graph.add_node(Country { name: "Venezuela", strength: Strength::Artillery }); 
        let brazil = graph.add_node(Country { name: "Brazil", strength: Strength::Artillery });
        let peru = graph.add_node(Country { name: "Peru", strength: Strength::Cavelry });
        let argentina = graph.add_node(Country { name: "Argentine", strength: Strength::Infantry });

        let egypt = graph.add_node(Country { name: "Egypt", strength: Strength::Infantry });
        let east_africa = graph.add_node(Country { name: "East Africa", strength: Strength::Artillery });
        let north_africa = graph.add_node(Country { name: "North Africa", strength: Strength::Infantry});
        let south_africa = graph.add_node(Country { name: "South Africa", strength: Strength::Artillery});
        let central_africa = graph.add_node(Country { name: "Central Africa", strength: Strength::Cavelry });
        let madagascar = graph.add_node(Country { name: "Madagascar", strength: Strength::Infantry });

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

