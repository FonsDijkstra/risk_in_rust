extern crate petgraph;

use board::card::Strength;
use board::country::petgraph::graph::{UnGraph, NodeIndex, NodeIndices};

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

        graph.add_edge(venezuela, peru, ());
        graph.add_edge(venezuela, brazil, ());
        graph.add_edge(peru, brazil, ());
        graph.add_edge(peru, argentina, ());
        graph.add_edge(brazil, argentina, ());

        Map { graph: graph }
    }

    pub fn all_countries(&self) ->  CountryIterator {
        self.graph.node_indices()
    }

    pub fn get_country(&self, id: CountryId) -> &Country {
        self.graph.node_weight(id).expect("unknown country id")
    }
}

