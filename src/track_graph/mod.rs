/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use serde::{Deserialize, Serialize};

mod algorithm;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ElementID(usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct UnitID(usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct GlobalID(usize, usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackGraph<EI, UI, GI> {
    unit_id: UI,
    elements: Vec<TrackElement<EI, GI>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackElement<EI, GI> {
    element_id: EI,
    connection: ElementConnections<EI, GI>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementConnections<EI, GI> {
    Track {
        x: EI,
        y: EI,
    },
    Derailer {
        from: EI,
        to: EI,
    },
    Point {
        tip: EI,
        normal: EI,
        reverse: EI,
        free_if_coupled_normal: Option<bool>,
    },
    Crossing {
        a: EI,
        b: EI,
        x: EI,
        y: EI,
    },
    DeadEnd {
        x: EI,
    },
    Signal {
        from: EI,
        to: EI,
    },
    OverlapEnd {
        x: EI,
        y: EI,
    },
    Block {
        x: EI,
        connected: Option<GI>,
    },
}
