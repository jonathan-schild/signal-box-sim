/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Error;

use crate::track_graph::{
    IdentifierMap, Node, TrackGraphID, extensions::Extensions, nodes::NodeType, regenerate_id_map,
};

mod algorithm;

pub struct TrackGraph {
    nodes: Vec<Node<TrackGraphID>>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TrackGraphBuilder<I>
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    nodes: Vec<Node<I>>,
    id_map: IdentifierMap<I>,
}

impl<I> TrackGraphBuilder<I>
where
    I: Debug + Default + Clone + Eq + Hash + Serialize + DeserializeOwned + 'static,
{
    #[must_use]
    pub fn new() -> Self {
        TrackGraphBuilder {
            nodes: Vec::new(),
            id_map: HashMap::new(),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        let nodes: Vec<_> = serde_json::from_str(json)?;
        let mut id_map = HashMap::new();

        regenerate_id_map(&nodes, &mut id_map);

        Ok(TrackGraphBuilder { nodes, id_map })
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self.nodes)
    }

    pub fn to_json_pretty(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(&self.nodes)
    }

    pub fn add_track(&mut self, id: I, x: I, y: I) -> bool {
        let node = Node {
            id,
            neighbours: NodeType::Track { x, y },
            extension: Extensions::new(),
        };
        self.add_node(node)
    }

    pub fn add_derailer(&mut self, id: I, from: I, to: I) -> bool {
        let node = Node {
            id,
            neighbours: NodeType::Derailer { from, to },
            extension: Extensions::new(),
        };
        self.add_node(node)
    }

    pub fn add_point(&mut self, id: I, tip: I, normal: I, reverse: I) -> bool {
        let node = Node {
            id,
            neighbours: NodeType::Point {
                tip,
                normal,
                reverse,
                free_if_coupled_normal: None,
            },
            extension: Extensions::new(),
        };
        self.add_node(node)
    }

    pub fn add_single_slip_point(
        &mut self,
        _id_ab: I,
        _id_xy: I,
        _a: I,
        _b: I,
        _x: I,
        _y: I,
    ) -> bool {
        todo!()
    }

    pub fn add_double_slip_point(
        &mut self,
        _id_ab: I,
        _id_xy: I,
        _a: I,
        _b: I,
        _x: I,
        _y: I,
    ) -> bool {
        todo!()
    }

    pub fn add_crossing(&mut self, id: I, a: I, b: I, x: I, y: I) -> bool {
        let node = Node {
            id,
            neighbours: NodeType::Crossing { a, b, x, y },
            extension: Extensions::new(),
        };
        self.add_node(node)
    }

    pub fn add_dead_end(&mut self, id: I, x: I) -> bool {
        let node = Node {
            id,
            neighbours: NodeType::DeadEnd { x },
            extension: Extensions::new(),
        };
        self.add_node(node)
    }

    pub fn add_signal(&mut self, _id: I, _from: I, _to: I) -> bool {
        todo!()
    }

    pub fn add_overlap_end(&mut self, _id: I, _x: I, _y: I) -> bool {
        todo!()
    }

    pub fn add_block(&mut self, id: I, x: I) -> bool {
        let node = Node {
            id,
            neighbours: NodeType::Block { x },
            extension: Extensions::new(),
        };
        self.add_node(node)
    }

    #[must_use]
    pub fn into_track_graph(self) -> (TrackGraph, HashMap<I, TrackGraphID>) {
        let nodes;
        let mut id_map = HashMap::new();

        let compacted_nodes: Vec<_> = self
            .nodes
            .into_iter()
            .filter(|n| !n.is_sentinel())
            .collect();

        regenerate_id_map(&compacted_nodes, &mut id_map);

        nodes = compacted_nodes
            .into_iter()
            .map(|n| n.transform(&id_map))
            .collect();

        (TrackGraph { nodes }, id_map)
    }

    fn add_node(&mut self, node: Node<I>) -> bool {
        let id = node.id.clone();

        if !self.id_map.contains_key(&id) {
            self.id_map.insert(id, TrackGraphID(self.nodes.len()));
            self.nodes.push(node);
            return true;
        }

        false
    }
}
