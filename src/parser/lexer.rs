/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{collections::VecDeque, io::BufRead, iter::Peekable, str::Chars};

use crate::{
    Error,
    parser::{Location, Token},
};

mod lexer_rules;

pub struct Lexer<R: BufRead> {
    source: R,
    location: Location,
}

impl<R: BufRead> Lexer<R> {
    pub fn new(source: R, file: String) -> Self {
        Self {
            source,
            location: Location {
                l: 0,
                c: 0,
                f: file,
            },
        }
    }

    pub fn scan(&mut self) -> Result<VecDeque<(Token, Location)>, Error> {
        let mut token_stream = VecDeque::new();

        loop {
            let mut token_line = self.scan_line()?;
            if token_line.is_empty() {
                break;
            }
            token_stream.append(&mut token_line);
        }

        Ok(token_stream)
    }

    pub fn scan_line(&mut self) -> Result<VecDeque<(Token, Location)>, Error> {
        let mut line_buffer = String::new();

        // EOF return
        if self.source.read_line(&mut line_buffer)? == 0 {
            return Ok(VecDeque::new());
        }

        let iter = line_buffer.chars().peekable();
        let mut lexer_state = LexerState::new(&mut self.location, iter);

        let tokens = lexer_state.scan_line()?;

        Ok(tokens)
    }
}

struct LexerState<'a> {
    token_stream: VecDeque<(Token, Location)>,
    current_location: &'a mut Location,
    buffer_line: Peekable<Chars<'a>>,
    name_buffer: String,
}

impl<'a> LexerState<'a> {
    fn new(location: &'a mut Location, buffer_line: Peekable<Chars<'a>>) -> Self {
        LexerState {
            token_stream: VecDeque::new(),
            current_location: location,
            buffer_line,
            name_buffer: String::new(),
        }
    }

    fn scan_line(&mut self) -> Result<VecDeque<(Token, Location)>, Error> {
        lexer_rules::scan_line(self)?;
        let mut token_line = VecDeque::new();
        token_line.append(&mut self.token_stream);
        self.token_stream.clear();
        Ok(token_line)
    }

    fn peek<'b, 'c>(&'b mut self) -> Option<&'c char>
    where
        'a: 'b,
        'b: 'c,
    {
        self.buffer_line.peek()
    }

    fn take(&mut self) -> bool {
        if let Some(c) = self.buffer_line.next() {
            self.name_buffer.push(c);
            true
        } else {
            false
        }
    }

    fn skip(&mut self) -> bool {
        self.buffer_line.next().is_some()
    }

    fn is_empty(&self) -> bool {
        self.name_buffer.is_empty()
    }

    fn buffer(&self) -> &str {
        self.name_buffer.as_str()
    }

    fn accept(&mut self, token: Token) {
        self.name_buffer.clear();
        self.token_stream
            .push_back((token, self.current_location.clone()));
    }
}
