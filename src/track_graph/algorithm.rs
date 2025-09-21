/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{fmt::Debug, hash::Hash};

use crate::track_graph::graph::TrackGraph;

impl<I> TrackGraph<I>
where
    I: Debug + Default + Clone + Eq + Hash + 'static,
{
    pub fn find_routes(&self, _start: I, _destination: I) -> Vec<Vec<I>> {
        todo!()
    }

    pub fn flank_protection(&self, _route: Vec<I>) -> Vec<Vec<I>> {
        todo!()
    }
}
