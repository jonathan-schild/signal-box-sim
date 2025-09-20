/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use serde::{Deserialize, Serialize};

use crate::track_graph::nodes::Node;
use std::{collections::HashMap, fmt, hash::Hash};

pub mod extensions;
pub mod graph;
mod nodes;

pub trait Identifier: fmt::Debug + Default + Clone + Eq + Hash + 'static {}

type IdentifierMap<I> = HashMap<I, InternalID>;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
struct InternalID(usize);

impl Identifier for InternalID {}

fn regenerate_id_map<I: Identifier>(nodes: &[Node<I>], id_map: &mut IdentifierMap<I>) {
    id_map.clear();

    for (i, n) in nodes.iter().enumerate() {
        if id_map.insert(n.id().clone(), InternalID(i)).is_some() {
            panic!("Id not unique!")
        }
    }
}

fn update_id<I: Identifier, J: Identifier>(id: &I, id_map: &HashMap<I, J>) -> J {
    id_map.get(id).expect("id mapping should exist").clone()
}
