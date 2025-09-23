/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

#[derive(Debug, Clone)]
pub struct Route<I>(pub Vec<RouteElement<I>>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RouteElement<I> {
    Element(I),
    Point(I, PointPosition, PointPosition),
    Opposed(I),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PointPosition {
    Tip,
    Normal,
    Reverse,
}
