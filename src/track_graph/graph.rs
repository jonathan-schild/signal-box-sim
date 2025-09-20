/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::collections::HashMap;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Error;

use crate::track_graph::{Identifier, IdentifierMap, InternalID, Node, regenerate_id_map};

pub struct TrackGraph<I: Identifier> {
    _nodes: Vec<Node<InternalID>>,
    _id_map: IdentifierMap<I>,
}

impl<I: Identifier> TrackGraph<I> {
    pub fn transform<J: Identifier>(&mut self, _id_map: &HashMap<I, J>) -> TrackGraph<J> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct TrackGraphBuilder<I: Identifier> {
    nodes: Vec<Node<I>>,
    _id_map: IdentifierMap<I>,
}

impl<I: Identifier + DeserializeOwned + Serialize> TrackGraphBuilder<I> {
    pub fn new() -> Self {
        TrackGraphBuilder {
            nodes: Vec::new(),
            _id_map: HashMap::new(),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        let nodes: Vec<_> = serde_json::from_str(json)?;
        let mut id_map = HashMap::new();

        regenerate_id_map(&nodes, &mut id_map);

        Ok(TrackGraphBuilder {
            nodes,
            _id_map: id_map,
        })
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self.nodes)
    }

    pub fn to_json_pretty(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(&self.nodes)
    }
}

impl<I: Identifier> From<TrackGraph<I>> for TrackGraphBuilder<I> {
    fn from(_value: TrackGraph<I>) -> Self {
        todo!()
    }
}

impl<I: Identifier> From<TrackGraphBuilder<I>> for TrackGraph<I> {
    fn from(value: TrackGraphBuilder<I>) -> Self {
        let nodes;
        let mut id_map = HashMap::new();

        let compacted_nodes: Vec<_> = value
            .nodes
            .into_iter()
            .filter(|n| !n.is_sentinel())
            .collect();

        regenerate_id_map(&compacted_nodes, &mut id_map);

        nodes = compacted_nodes
            .into_iter()
            .map(|n| n.transform(&id_map))
            .collect();

        Self {
            _nodes: nodes,
            _id_map: id_map,
        }
    }
}
