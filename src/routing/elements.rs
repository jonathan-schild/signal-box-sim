use crate::{
    Error,
    routing::{ElementID, NO_PREVIOUS_ELEMENT, Pathfinder, TrainRoute},
};

pub enum Elements {
    Point(Point),
    Track(Track),
    DeadEndTrack(DeadEndTrack),
}

impl TrainRoute for Elements {
    fn train_route(&self, pathfinder: &dyn Pathfinder) {
        match self {
            Elements::Point(point) => point.train_route(pathfinder),
            Elements::Track(track) => track.train_route(pathfinder),
            Elements::DeadEndTrack(dead_end_track) => dead_end_track.train_route(pathfinder),
        }
    }
}

pub struct Point {
    pub(crate) id: ElementID,
    pub(crate) straight: ElementID,
    pub(crate) curved: ElementID,
    pub(crate) trunk: ElementID,
}

impl TrainRoute for Point {
    fn train_route(&self, pathfinder: &dyn Pathfinder) {
        debug_assert_eq!(self.id, pathfinder.current_position());

        let prev = pathfinder.previous_position();
        if prev == self.trunk {
            pathfinder.next_elements(&[self.curved, self.straight]);
        } else if prev == self.curved || prev == self.straight {
            pathfinder.next_elements(&[self.trunk]);
        } else {
            pathfinder.error(Error::NoConnectionToPreviousElement);
        }
    }
}

pub struct Track {
    pub(crate) id: ElementID,
    pub(crate) neighbour0: ElementID,
    pub(crate) neighbour1: ElementID,
}

impl TrainRoute for Track {
    fn train_route(&self, pathfinder: &dyn Pathfinder) {
        debug_assert_eq!(self.id, pathfinder.current_position());

        let prev = pathfinder.previous_position();
        if prev == self.neighbour0 {
            pathfinder.next_elements(&[self.neighbour1]);
        } else if prev == self.neighbour1 {
            pathfinder.next_elements(&[self.neighbour0]);
        } else {
            pathfinder.error(Error::NoConnectionToPreviousElement);
        }
    }
}

pub struct DeadEndTrack {
    pub(crate) id: ElementID,
    pub(crate) neighbour: ElementID,
}

impl TrainRoute for DeadEndTrack {
    fn train_route(&self, pathfinder: &dyn Pathfinder) {
        debug_assert_eq!(self.id, pathfinder.current_position());

        let prev = pathfinder.previous_position();
        if prev == self.neighbour {
            pathfinder.next_elements(&[]);
        } else if prev == NO_PREVIOUS_ELEMENT {
            pathfinder.next_elements(&[self.neighbour]);
        } else {
            pathfinder.error(Error::NoConnectionToPreviousElement);
        }
    }
}
