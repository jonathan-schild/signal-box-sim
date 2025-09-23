/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::io;

use thiserror::Error;

pub mod routes;
pub mod track_graph;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Unknown Error")]
    Unknown,
}
