/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use crate::Error;

pub mod elements;
pub mod pathfinder;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ElementID(usize);

impl From<usize> for ElementID {
    fn from(value: usize) -> Self {
        ElementID(value)
    }
}

const NO_PREVIOUS_ELEMENT: ElementID = ElementID(usize::MAX);

pub trait TrainRoute {
    fn train_route(&self, pathfinder: &dyn Pathfinder);
}

pub trait Pathfinder {
    fn current_position(&self) -> ElementID;
    fn previous_position(&self) -> ElementID;
    fn next_elements(&self, elements: &[ElementID]);
    fn error(&self, err: Error);
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::routing::{
        elements::{DeadEndTrack, Elements, Point},
        pathfinder,
    };

    #[test]
    fn simple_station() {
        let elements = vec![
            Elements::DeadEndTrack(DeadEndTrack {
                id: 0.into(),
                neighbour: 1.into(),
            }),
            Elements::Point(Point {
                id: 1.into(),
                straight: 2.into(),
                curved: 3.into(),
                trunk: 0.into(),
            }),
            Elements::DeadEndTrack(DeadEndTrack {
                id: 2.into(),
                neighbour: 1.into(),
            }),
            Elements::DeadEndTrack(DeadEndTrack {
                id: 3.into(),
                neighbour: 1.into(),
            }),
        ];

        let elements = Rc::new(RefCell::new(elements));

        let path_to_2 = pathfinder::Pathfinder::find(0.into(), 2.into(), elements.clone());
        let path_to_3 = pathfinder::Pathfinder::find(0.into(), 3.into(), elements.clone());
        let path_to_0 = pathfinder::Pathfinder::find(3.into(), 0.into(), elements.clone());

        assert_eq!(path_to_2, Some(vec![0.into(), 1.into(), 2.into()]));
        assert_eq!(path_to_3, Some(vec![0.into(), 1.into(), 3.into()]));
        assert_eq!(path_to_0, Some(vec![3.into(), 1.into(), 0.into()]));
    }
}
