/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::track_graph::{extensions::Extensions, update_id};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Node<I>
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    pub(super) id: I,
    pub(super) neighbours: NodeType<I>,
    #[serde(skip)]
    pub(super) extension: Extensions,
}

impl<I> Node<I>
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    pub fn id(&self) -> &I {
        &self.id
    }

    pub fn is_sentinel(&self) -> bool {
        let NodeType::Sentinel = self.neighbours else {
            return false;
        };

        true
    }

    pub fn transform<J>(self, id_map: &HashMap<I, J>) -> Node<J>
    where
        J: Debug + Default + Clone + Eq + Hash + 'static,
    {
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

impl<I> PartialEq for Node<I>
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.neighbours == other.neighbours
    }
}

impl<I> Eq for Node<I> where I: Debug + Default + Clone + Eq + Hash + 'static {}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) enum NodeType<I> {
    Sentinel,
    Track {
        x: I,
        y: I,
    },
    Derailer {
        from: I,
        to: I,
    },
    Point {
        tip: I,
        normal: I,
        reverse: I,
        free_if_coupled_normal: Option<bool>,
    },
    Crossing {
        a: I,
        b: I,
        x: I,
        y: I,
    },
    DeadEnd {
        x: I,
    },
    Signal {
        from: I,
        to: I,
    },
    OverlapEnd {
        x: I,
        y: I,
    },
    Block {
        x: I,
    },
}

impl<I> NodeType<I>
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    fn transform<J>(self, id_map: &HashMap<I, J>) -> NodeType<J>
    where
        J: Debug + Default + Clone + Eq + Hash + 'static,
    {
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
                free_if_coupled_normal,
            } => NodeType::Point {
                tip: update_id(tip, id_map),
                normal: update_id(normal, id_map),
                reverse: update_id(reverse, id_map),
                free_if_coupled_normal: *free_if_coupled_normal,
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
