/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::io;

use thiserror::Error;

pub mod model;
pub mod parser;
pub mod svg_render;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Unexpected symbol: {0}; expected: {1}")]
    UnexpectedSymbol(char, String),
    #[error("Expected: {0}")]
    ExpectedToken(String),
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),
    #[error("Unknown Error")]
    Unknown,
}
