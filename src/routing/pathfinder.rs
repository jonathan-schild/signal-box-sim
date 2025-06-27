/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{cell::RefCell, rc::Rc};

use crate::routing::{ElementID, NO_PREVIOUS_ELEMENT, TrainRoute, elements::Elements};

pub struct Pathfinder {
    inner: RefCell<InnerPathfinder>,
}

impl Pathfinder {
    pub fn find(
        start: ElementID,
        destination: ElementID,
        elements: Rc<RefCell<Vec<Elements>>>,
    ) -> Option<Vec<ElementID>> {
        let pathfinder = Pathfinder {
            inner: RefCell::new(InnerPathfinder {
                current_path_index: 0,
                paths: vec![vec![start]],
            }),
        };

        pathfinder.inner_iter(destination, elements)
    }

    pub fn inner_iter(
        mut self,
        destination: ElementID,
        elements: Rc<RefCell<Vec<Elements>>>,
    ) -> Option<Vec<ElementID>> {
        while let Some(eid) = self.inner.get_mut().next(destination) {
            let current_element = &elements.borrow()[eid.0];
            current_element.train_route(&self);
        }
        self.inner.into_inner().result()
    }
}

impl super::Pathfinder for Pathfinder {
    fn current_position(&self) -> ElementID {
        self.inner.borrow().current_position()
    }

    fn previous_position(&self) -> ElementID {
        self.inner.borrow().previous_position()
    }

    fn next_elements(&self, elements: &[ElementID]) {
        self.inner.borrow_mut().next_elements(elements);
    }

    fn error(&self, _err: crate::Error) {
        todo!()
    }
}

struct InnerPathfinder {
    current_path_index: usize,
    paths: Vec<Vec<ElementID>>,
}

impl InnerPathfinder {
    fn next(&mut self, destination: ElementID) -> Option<ElementID> {
        while let Some(current) = self.paths.get(self.current_path_index) {
            if let Some(last) = current.last() {
                if last == &destination {
                    self.current_path_index += 1;
                } else {
                    return Some(*last);
                }
            } else {
                self.current_path_index += 1;
            }
        }

        None
    }

    fn result(self) -> Option<Vec<ElementID>> {
        let mut paths: Vec<Vec<ElementID>> =
            self.paths.into_iter().filter(|v| !v.is_empty()).collect();
        if paths.is_empty() {
            None
        } else {
            Some(paths.remove(0))
        }
    }

    fn current_position(&self) -> ElementID {
        *self.paths[self.current_path_index].last().unwrap()
    }

    fn previous_position(&self) -> ElementID {
        let current = &self.paths[self.current_path_index];
        if current.len() < 2 {
            NO_PREVIOUS_ELEMENT
        } else if let Some(prev) = current.get(current.len() - 2) {
            *prev
        } else {
            NO_PREVIOUS_ELEMENT
        }
    }

    fn next_elements(&mut self, elements: &[ElementID]) {
        if elements.len() == 0 {
            self.paths[self.current_path_index].clear();
        } else if elements.len() == 1 {
            self.paths[self.current_path_index].push(elements[0]);
        } else {
            let temp = self.paths[self.current_path_index].clone();
            self.paths[self.current_path_index].clear();
            for e in elements {
                let mut temp_clone = temp.clone();
                temp_clone.push(*e);
                self.paths.push(temp_clone);
            }
        }
    }
}
