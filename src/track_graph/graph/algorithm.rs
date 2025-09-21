/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::collections::VecDeque;

use crate::track_graph::{TrackGraphID, graph::TrackGraph, nodes::NodeType};

impl TrackGraph {
    #[must_use]
    pub fn find_routes(
        &self,
        start: TrackGraphID,
        destination: TrackGraphID,
    ) -> Vec<Vec<TrackGraphID>> {
        let NodeType::Signal { from: _, to } = self.nodes.get(start.0).unwrap().neighbours else {
            return Vec::new();
        };

        let mut routes = Vec::new();

        let mut work_list = VecDeque::from(vec![vec![start, to]]);

        while let Some(mut route) = work_list.pop_front() {
            let previous_node = route.get(route.len() - 2).unwrap();
            let current_node = route.last().unwrap();
            let neighbors = &self.nodes.get(current_node.0).unwrap().neighbours;

            match neighbors {
                NodeType::Sentinel => panic!(),
                NodeType::Track { x, y } => {
                    if previous_node == x {
                        route.push(*y);
                    } else if previous_node == y {
                        route.push(*x);
                    } else {
                        panic!();
                    }
                }
                NodeType::Derailer { from, to } => {
                    if previous_node == from {
                        route.push(*to);
                    } else if previous_node == to {
                        route.push(*from);
                    } else {
                        panic!()
                    }
                }
                NodeType::Point {
                    tip,
                    normal,
                    reverse,
                    free_if_coupled_normal: _,
                } => {
                    if previous_node == tip {
                        let mut new_route = route.clone();
                        new_route.push(*reverse);
                        work_list.push_back(new_route);
                        route.push(*normal);
                    } else if previous_node == normal || previous_node == reverse {
                        route.push(*tip);
                    } else {
                        panic!()
                    }
                }
                NodeType::Crossing { a, b, x, y } => {
                    if previous_node == a {
                        route.push(*b);
                    } else if previous_node == b {
                        route.push(*a);
                    } else if previous_node == x {
                        route.push(*y);
                    } else if previous_node == y {
                        route.push(*x);
                    } else {
                        panic!()
                    }
                }
                NodeType::Signal { from, to } => {
                    if previous_node == from {
                        route.push(*to);
                    } else if previous_node == to {
                        route.pop();
                        route.push(*from);
                    } else {
                        panic!()
                    }
                }
                NodeType::OverlapEnd { x: _, y: _ } => (),
                NodeType::DeadEnd { x } | NodeType::Block { x } => {
                    assert!((previous_node == x && current_node == &destination),);
                }
            }

            if *route.last().unwrap() == destination {
                routes.push(route);
            } else {
                work_list.push_back(route);
            }
        }

        routes
    }

    #[must_use]
    pub fn find_flank_protection_path(&self, _route: Vec<TrackGraphID>) -> Vec<Vec<TrackGraphID>> {
        todo!()
    }
}
