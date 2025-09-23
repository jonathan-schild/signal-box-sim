/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::collections::VecDeque;

use crate::{
    routes::{Route, RouteElement},
    track_graph::{ElementConnections, ElementID, GlobalID, TrackGraph, UnitID},
};

impl TrackGraph<ElementID, UnitID, GlobalID> {
    pub fn find_routes(&self, start: ElementID, destination: ElementID) -> Vec<Route<ElementID>> {
        let ElementConnections::Signal { from: _, to } =
            self.elements.get(start.0).unwrap().connection
        else {
            panic!()
        };

        let mut route = vec![vec![RouteElement::Element(start)]];
        let mut work_list = VecDeque::from(vec![(to, start, 0)]);

        while let Some((current_element, previous_element, route_index)) = work_list.pop_front() {
            let neighbors = &self.elements.get(current_element.0).unwrap().connection;

            match neighbors {
                _ => todo!(),
            }
        }

        todo!()
    }
}
