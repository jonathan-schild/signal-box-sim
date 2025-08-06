/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

// TODO remove
#![allow(dead_code)]

use std::collections::HashMap;

use petgraph::{algo::connected_components, graph::UnGraph};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum PointPosition {
    Normal,
    Reverse,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum SignalAspects {
    Proceed,
    Kennlicht,
    Sh1,
    Zs1,
    Zs6,
    Zs7,
    Zs8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    location: Location,
    elements: Vec<Element>,
    addons: Vec<AddOns>,
    #[serde(skip)]
    graph: UnGraph<String, &'static str>,
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
            && self.elements == other.elements
            && self.addons == other.addons
    }
}

impl Eq for Model {}

impl Model {
    fn validate_graph(&mut self) -> bool {
        let mut name_map = HashMap::new();

        for node in &self.elements {
            if name_map.contains_key(node.name()) {
                return false;
            } else {
                name_map.insert(node.name(), self.graph.add_node(node.name().to_string()))
            };
        }

        for node in &self.elements {
            for edge in node.edges() {
                self.graph.update_edge(
                    *name_map.get(node.name()).unwrap(),
                    *name_map.get(edge).unwrap(),
                    "",
                );
            }
        }

        if connected_components(&self.graph) != 1 {
            return false;
        }

        true
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Location {
    name: String,
    abbreviation: String,
    neighbours: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum Element {
    Point(Point),
    Crossing(Crossing),
    Track(Track),
}

impl Element {
    fn name(&self) -> &str {
        match self {
            Element::Point(point) => &point.name,
            Element::Crossing(crossing) => &crossing.name,
            Element::Track(track) => &track.name,
        }
    }

    fn edges(&self) -> Vec<&str> {
        let mut result = vec![];
        match self {
            Element::Point(point) => {
                result.push(point.tip.as_str());
                result.push(point.normal.as_str());
                if let Some(reverse) = &point.reverse {
                    result.push(reverse.as_str());
                }
            }
            Element::Crossing(crossing) => {
                result.push(&crossing.path_a);
                result.push(&crossing.path_b);
                result.push(&crossing.path_c);
                result.push(&crossing.path_d);
            }
            Element::Track(track) => {
                if let Some(path) = &track.path {
                    result.push(path.as_str());
                }
            }
        }
        result
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum AddOns {
    OverlapEnd(OverlapEnd),
    RouteDestination(RouteDestination),
    Signal(Signal),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Point {
    name: String,
    tip: String,
    normal: String,
    reverse: Option<String>,
    coupled: Option<String>,
    unrestricted: Option<PointPosition>,
    default: Option<PointPosition>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Crossing {
    name: String,
    path_a: String,
    path_b: String,
    path_c: String,
    path_d: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Track {
    name: String,
    path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct OverlapEnd {
    name: String,
    path_a: String,
    path_b: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct RouteDestination {
    name: String,
    path_from: String,
    path_to: Option<String>,
    opposite_track: Option<bool>,
    train: bool,
    shunting: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Signal {
    name: String,
    path_from: String,
    path_to: Option<String>,
    overlaps: Vec<String>,
    coupled: Vec<String>,
    opposite_track: Option<bool>,
    aspects: Vec<SignalAspects>,
}

#[cfg(test)]
mod tests {
    use crate::model::{Element, Location, Model, Point, PointPosition, Track};

    #[test]
    fn verify_graph_edge_degree() {
        // TODO
    }

    #[test]
    fn verify_graph_component_number() {
        let mut elements = vec![];
        elements.push(Element::Track(Track {
            name: "v.u.n A-Stadt".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "1".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "2".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "3".to_string(),
            path: None,
        }));

        elements.push(Element::Point(Point {
            name: "W1".to_string(),
            tip: "v.u.n A-Stadt".to_string(),
            normal: "1".to_string(),
            reverse: Some("2".to_string()),
            coupled: None,
            unrestricted: None,
            default: None,
        }));

        let mut model = Model {
            location: Location {
                name: "B-Stadt".to_string(),
                abbreviation: "SBST".to_string(),
                neighbours: vec!["A-Stadt".to_string()],
            },
            elements,
            addons: vec![],
            graph: Default::default(),
        };

        assert!(!model.validate_graph());
    }

    #[test]
    fn verify_graph_duplicated_node() {
        let mut elements = vec![];
        elements.push(Element::Track(Track {
            name: "v.u.n A-Stadt".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "1".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "2".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "2".to_string(),
            path: None,
        }));

        elements.push(Element::Point(Point {
            name: "W1".to_string(),
            tip: "v.u.n A-Stadt".to_string(),
            normal: "1".to_string(),
            reverse: Some("2".to_string()),
            coupled: None,
            unrestricted: None,
            default: None,
        }));

        let mut model = Model {
            location: Location {
                name: "B-Stadt".to_string(),
                abbreviation: "SBST".to_string(),
                neighbours: vec!["A-Stadt".to_string()],
            },
            elements,
            addons: vec![],
            graph: Default::default(),
        };

        assert!(!model.validate_graph());
    }

    #[test]
    fn verify_graph_unknown_node() {
        let mut elements = vec![];
        elements.push(Element::Track(Track {
            name: "1".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "2".to_string(),
            path: None,
        }));

        elements.push(Element::Point(Point {
            name: "W1".to_string(),
            tip: "v.u.n A-Stadt".to_string(),
            normal: "1".to_string(),
            reverse: Some("2".to_string()),
            coupled: None,
            unrestricted: None,
            default: None,
        }));

        let mut model = Model {
            location: Location {
                name: "B-Stadt".to_string(),
                abbreviation: "SBST".to_string(),
                neighbours: vec!["A-Stadt".to_string()],
            },
            elements,
            addons: vec![],
            graph: Default::default(),
        };

        assert!(!model.validate_graph());
    }

    #[test]
    fn serde() {
        let mut elements = vec![];

        elements.push(Element::Track(Track {
            name: "n. A-Dorf".to_string(),
            path: Some("1-1".to_string()),
        }));
        elements.push(Element::Track(Track {
            name: "1-1".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "1-2".to_string(),
            path: Some("1".to_string()),
        }));
        elements.push(Element::Track(Track {
            name: "1".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "v.u.n. C-Stadt".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "v. A-Stadt".to_string(),
            path: Some("2-1".to_string()),
        }));
        elements.push(Element::Track(Track {
            name: "2-1".to_string(),
            path: None,
        }));
        elements.push(Element::Track(Track {
            name: "2-2".to_string(),
            path: Some("2".to_string()),
        }));
        elements.push(Element::Track(Track {
            name: "2".to_string(),
            path: None,
        }));
        elements.push(Element::Point(Point {
            name: "W1".to_string(),
            tip: "2-1".to_string(),
            normal: "2-2".to_string(),
            reverse: Some("W2".to_string()),
            coupled: None,
            unrestricted: None,
            default: Some(PointPosition::Normal),
        }));
        elements.push(Element::Point(Point {
            name: "W2".to_string(),
            tip: "1-2".to_string(),
            normal: "1-1".to_string(),
            reverse: Some("W1".to_string()),
            coupled: None,
            unrestricted: None,
            default: Some(PointPosition::Normal),
        }));
        elements.push(Element::Point(Point {
            name: "W3".to_string(),
            tip: "v.u.n. C-Stadt".to_string(),
            normal: "2".to_string(),
            reverse: Some("1".to_string()),
            coupled: None,
            unrestricted: None,
            default: None,
        }));

        let mut model = Model {
            location: Location {
                name: "B-Stadt".to_string(),
                abbreviation: "SBST".to_string(),
                neighbours: vec!["A-Dorf".to_string(), "B-Stadt".to_string()],
            },
            elements,
            addons: vec![],
            graph: Default::default(),
        };

        let json = serde_json::to_string_pretty(&model).unwrap();

        assert!(model.validate_graph());
        assert_eq!(model, serde_json::from_str(&json).unwrap());
    }
}
