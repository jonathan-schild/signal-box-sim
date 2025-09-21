/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use serde::{Deserialize, Serialize};

use crate::track_graph::nodes::Node;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub mod extensions;
pub mod graph;
mod nodes;

type IdentifierMap<I> = HashMap<I, TrackGraphID>;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct TrackGraphID(usize);

fn regenerate_id_map<I>(nodes: &[Node<I>], id_map: &mut IdentifierMap<I>)
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    id_map.clear();

    for (i, n) in nodes.iter().enumerate() {
        assert!(
            id_map.insert(n.id().clone(), TrackGraphID(i)).is_none(),
            "Id not unique!"
        );
    }
}

fn update_id<I, J>(id: &I, id_map: &HashMap<I, J>) -> J
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
    J: Debug + Default + Clone + Eq + Hash + 'static,
{
    id_map.get(id).expect("id mapping should exist").clone()
}
