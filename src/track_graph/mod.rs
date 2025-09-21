/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use serde::{Deserialize, Serialize};

use crate::track_graph::nodes::Node;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

mod algorithm;
pub mod extensions;
pub mod graph;
mod nodes;

// pub trait Identifier: fmt::Debug + Default + Clone + Eq + Hash + 'static {}

type IdentifierMap<I> = HashMap<I, InternalID>;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
struct InternalID(usize);

fn regenerate_id_map<I>(nodes: &[Node<I>], id_map: &mut IdentifierMap<I>)
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    id_map.clear();

    for (i, n) in nodes.iter().enumerate() {
        if id_map.insert(n.id().clone(), InternalID(i)).is_some() {
            panic!("Id not unique!")
        }
    }
}

fn update_id<I, J>(id: &I, id_map: &HashMap<I, J>) -> J
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
    J: Debug + Default + Clone + Eq + Hash + 'static,
{
    id_map.get(id).expect("id mapping should exist").clone()
}
