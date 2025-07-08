/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

mod inner;

pub use inner::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Elements {
    Track(Track),
    Point(Point),
    SlipPoint(SlipPoint),
    MainSignal(MainSignal),
    DistantSignal(DistantSignal),
    ShuntingSignal(ShuntingSignal),
    TrainRouteEnd(TrainRouteEnd),
}
