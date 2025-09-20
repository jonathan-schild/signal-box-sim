/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::track_graph::{Identifier, extensions::Extensions, update_id};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Node<I: Identifier> {
    id: I,
    neighbours: NodeType<I>,
    #[serde(skip)]
    extension: Extensions,
}

impl<I: Identifier> Node<I> {
    pub fn id(&self) -> &I {
        &self.id
    }

    pub fn is_sentinel(&self) -> bool {
        let NodeType::Sentinel = self.neighbours else {
            return false;
        };

        true
    }

    pub fn transform<J: Identifier>(self, id_map: &HashMap<I, J>) -> Node<J> {
        let mut extension = self.extension;
        extension.insert(self.id.clone());

        let id = update_id(&self.id, id_map);

        Node {
            id,
            neighbours: self.neighbours.transform(id_map),
            extension,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) enum NodeType<I> {
    Sentinel,
    Track { x: I, y: I },
    Derailer { from: I, to: I },
    Point { tip: I, normal: I, reverse: I },
    Crossing { a: I, b: I, x: I, y: I },
    DeadEnd { x: I },
    Signal { from: I, to: I },
    OverlapEnd { x: I, y: I },
    Block { x: I },
}

impl<I: Identifier> NodeType<I> {
    fn transform<J: Identifier>(self, id_map: &HashMap<I, J>) -> NodeType<J> {
        match &self {
            NodeType::Sentinel => NodeType::Sentinel,
            NodeType::Track { x, y } => NodeType::Track {
                x: update_id(x, id_map),
                y: update_id(y, id_map),
            },
            NodeType::Derailer { from, to } => NodeType::Derailer {
                from: update_id(from, id_map),
                to: update_id(to, id_map),
            },
            NodeType::Point {
                tip,
                normal,
                reverse,
            } => NodeType::Point {
                tip: update_id(tip, id_map),
                normal: update_id(normal, id_map),
                reverse: update_id(reverse, id_map),
            },
            NodeType::Crossing { a, b, x, y } => NodeType::Crossing {
                a: update_id(a, id_map),
                b: update_id(b, id_map),
                x: update_id(x, id_map),
                y: update_id(y, id_map),
            },
            NodeType::DeadEnd { x } => NodeType::DeadEnd {
                x: update_id(x, id_map),
            },
            NodeType::Signal { from, to } => NodeType::Signal {
                from: update_id(from, id_map),
                to: update_id(to, id_map),
            },
            NodeType::OverlapEnd { x, y } => NodeType::OverlapEnd {
                x: update_id(x, id_map),
                y: update_id(y, id_map),
            },
            NodeType::Block { x } => NodeType::Block {
                x: update_id(x, id_map),
            },
        }
    }
}
