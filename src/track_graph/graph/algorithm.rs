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
            panic!()
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
                        // route.pop();
                        route.push(*from);
                    } else {
                        panic!()
                    }
                }
                NodeType::OverlapEnd { x: _, y: _ } => todo!(),
                NodeType::DeadEnd { x } | NodeType::Block { x } => {
                    assert!(previous_node == x);
                    if current_node != &destination {
                        continue;
                    }
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

#[cfg(test)]
mod test {
    use crate::track_graph::{Node, extensions::Extensions};

    use super::*;

    #[test]
    fn test_find_routes() {
        /*
         * Track Graph:
         *                                                      / --- Asig P2 [4] <- --- 2 [6] --- End 2 [8] ---|
         *                                                     /
         * Block v.u.n. A-Stadt [0] --- Esig A [1] -> --- W 1 [2] --- Asig P1 [3] <- --- 1 [5] --- End 1 [7] ---|
         */

        let graph = TrackGraph {
            nodes: vec![
                // Block v.u.n. A-Stadt
                Node {
                    id: TrackGraphID(0),
                    neighbours: NodeType::Block { x: 1.into() },
                    extension: Extensions::new(),
                },
                // Esig A
                Node {
                    id: TrackGraphID(1),
                    neighbours: NodeType::Signal {
                        from: 0.into(),
                        to: 2.into(),
                    },
                    extension: Extensions::new(),
                },
                // W1
                Node {
                    id: TrackGraphID(2),
                    neighbours: NodeType::Point {
                        tip: 1.into(),
                        normal: 3.into(),
                        reverse: 4.into(),
                        free_if_coupled_normal: None,
                    },
                    extension: Extensions::new(),
                },
                // Asig P1
                Node {
                    id: TrackGraphID(3),
                    neighbours: NodeType::Signal {
                        from: 5.into(),
                        to: 2.into(),
                    },
                    extension: Extensions::new(),
                },
                // Asig P2
                Node {
                    id: TrackGraphID(4),
                    neighbours: NodeType::Signal {
                        from: 6.into(),
                        to: 2.into(),
                    },
                    extension: Extensions::new(),
                },
                // Track 1
                Node {
                    id: TrackGraphID(5),
                    neighbours: NodeType::Track {
                        x: 3.into(),
                        y: 7.into(),
                    },
                    extension: Extensions::new(),
                },
                // Track 2
                Node {
                    id: TrackGraphID(6),
                    neighbours: NodeType::Track {
                        x: 4.into(),
                        y: 8.into(),
                    },
                    extension: Extensions::new(),
                },
                // End 1
                Node {
                    id: TrackGraphID(7),
                    neighbours: NodeType::DeadEnd { x: 5.into() },
                    extension: Extensions::new(),
                },
                // End 2
                Node {
                    id: TrackGraphID(8),
                    neighbours: NodeType::DeadEnd { x: 6.into() },
                    extension: Extensions::new(),
                },
            ],
        };

        let rout_to_1 = graph.find_routes(1.into(), 7.into());
        assert_eq!(
            rout_to_1,
            [[
                TrackGraphID(1),
                TrackGraphID(2),
                TrackGraphID(3),
                TrackGraphID(5),
                TrackGraphID(7)
            ]]
        );

        let rout_to_2 = graph.find_routes(1.into(), 8.into());
        assert_eq!(
            rout_to_2,
            [[
                TrackGraphID(1),
                TrackGraphID(2),
                TrackGraphID(4),
                TrackGraphID(6),
                TrackGraphID(8)
            ]]
        );

        let rout_to_a_1 = graph.find_routes(3.into(), 0.into());
        assert_eq!(
            rout_to_a_1,
            [[
                TrackGraphID(3),
                TrackGraphID(2),
                TrackGraphID(1),
                TrackGraphID(0)
            ]]
        );

        let rout_to_a_2 = graph.find_routes(4.into(), 0.into());
        assert_eq!(
            rout_to_a_2,
            [[
                TrackGraphID(4),
                TrackGraphID(2),
                TrackGraphID(1),
                TrackGraphID(0)
            ]]
        );
    }
}
