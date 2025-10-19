/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use bitflags::bitflags;

pub struct Event {
    from: ElementID,
    to: ElementID,
}

pub trait EventProcessor {
    fn process(&mut self, event: &mut Event) {}
}

#[derive(Debug, Clone, Copy)]
pub struct ElementID {
    pub id: usize,
}

#[derive(Debug)]
pub struct PointGroup {
    occupied: bool,
    position: PointPosition,
    tip: ElementID,
    normal: ElementID,
    reverse: ElementID,
    route_normal: bool,
    route_reverse: bool,
}

#[derive(Debug)]
pub struct ShuntingRouteGroup {
    from: ElementID,
    to: ElementID,
    active: ElementID,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    struct PointPosition: u8 {
        const Normal            = 1 << 0;
        const Reverse           = 1 << 1;
        const RouteNormal       = 1 << 2;
        const RouteReverse      = 1 << 3;
        const PreferNormal      = 1 << 4;
        const PreferReverse     = 1 << 5;
        const TurningNormal     = 1 << 6;
        const TurningReverse    = 1 << 7;
    }
}
