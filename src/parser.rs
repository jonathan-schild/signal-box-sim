/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::io::BufRead;

use crate::{
    Error,
    parser::{
        ast::{AbstractSyntaxTree, Parser},
        lexer::Lexer,
    },
};

mod lexer;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Token {
    LBRACKET,
    RBRACKET,
    DASH,
    SEMICOLON,
    COMMA,
    W_POINT,
    T_TUNNEL,
    K_CROSSING,
    DKW_DOUBLE_SLIP_POINT,
    EKW_SINGLE_SLIP_POINT,
    GS_DERAILER,
    IDENTIFIER(String),
}

#[derive(Debug, Clone)]
pub struct Location {
    pub l: usize,
    pub c: usize,
    pub f: String,
}

pub fn parse<R: BufRead>(source: R, file: String) -> Result<AbstractSyntaxTree, Error> {
    let mut lexer = Lexer::new(source, file);
    let mut tokens = lexer.scan()?;
    let ast = Parser::parse(&mut tokens)?;
    Ok(ast)
}

mod ast {
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        hash::Hash,
    };

    use crate::{
        Error,
        parser::{Location, Token},
    };

    pub struct AbstractSyntaxTree {
        pub track_line: Vec<Element>,
        pub id_map: HashMap<String, u64>,
    }

    pub struct Parser<'a> {
        tokens: &'a mut VecDeque<(Token, Location)>,
        track_line: Vec<Element>,
        id_map: HashMap<String, u64>,
    }

    impl<'a> Parser<'a> {
        pub fn parse<'b>(
            tokens: &'b mut VecDeque<(Token, Location)>,
        ) -> Result<AbstractSyntaxTree, Error>
        where
            'b: 'a,
        {
            let mut ast = Parser {
                tokens,
                track_line: Vec::new(),
                id_map: HashMap::new(),
            };
            ast.parse_track_line()?;

            Ok(AbstractSyntaxTree {
                track_line: ast.track_line,
                id_map: ast.id_map,
            })
        }

        fn parse_track_line(&mut self) -> Result<(), Error> {
            while !self.tokens.is_empty() {
                let element = self.parse_element()?;
                let current = &self.tokens.front().ok_or(Error::UnexpectedEOF)?.0;

                if let Token::DASH = current {
                    self.track_line.push(Element {
                        element,
                        line_end: false,
                        connected_to_next: true,
                    });
                    self.tokens.pop_front();
                } else if let Token::SEMICOLON = current {
                    self.track_line.push(Element {
                        element,
                        line_end: true,
                        connected_to_next: false,
                    });
                    self.tokens.pop_front();
                }
            }

            Ok(())
        }

        fn parse_element(&mut self) -> Result<ElementNode, Error> {
            let (token, location) = self.tokens.pop_front().ok_or(Error::UnexpectedEOF)?;

            let mut element = match token {
                Token::LBRACKET => self.parse_track(),
                Token::W_POINT => self.parse_point(),
                Token::T_TUNNEL => self.parse_tunnel(),
                Token::K_CROSSING => self.parse_crossing(),
                Token::DKW_DOUBLE_SLIP_POINT => self.parse_double_slip_point(),
                Token::EKW_SINGLE_SLIP_POINT => self.parse_single_slip_point(),
                Token::GS_DERAILER => self.parse_derailer(),
                _ => Err(Error::ExpectedToken(
                    r#""[", "W", "DKW", "EKW", "Gs", "K" or "T""#.to_string(),
                )),
            }?;

            element.set_location(location);

            Ok(element)
        }

        fn parse_track(&mut self) -> Result<ElementNode, Error> {
            let numeric_id = self.parse_identifier();

            todo!()
        }

        fn parse_point(&mut self) -> Result<ElementNode, Error> {
            todo!()
        }

        fn parse_tunnel(&mut self) -> Result<ElementNode, Error> {
            todo!()
        }

        fn parse_crossing(&mut self) -> Result<ElementNode, Error> {
            todo!()
        }

        fn parse_double_slip_point(&mut self) -> Result<ElementNode, Error> {
            todo!()
        }

        fn parse_single_slip_point(&mut self) -> Result<ElementNode, Error> {
            todo!()
        }

        fn parse_derailer(&mut self) -> Result<ElementNode, Error> {
            todo!()
        }

        fn parse_identifier(&mut self) -> Result<u64, Error> {
            let (id, _) = self.tokens.pop_front().ok_or(Error::UnexpectedEOF)?;

            if let Token::IDENTIFIER(id) = id {
                let numeric_id = if let Some(numeric_id) = self.id_map.get(&id) {
                    *numeric_id
                } else {
                    let numeric_id = self.id_map.len() as u64;
                    let assert = self.id_map.insert(id, numeric_id);
                    debug_assert!(assert.is_none());
                    numeric_id
                };

                Ok(numeric_id)
            } else {
                Err(Error::ExpectedToken("<Identifier>".to_string()))
            }
        }
    }

    pub struct Element {
        pub element: ElementNode,
        pub line_end: bool,
        pub connected_to_next: bool,
    }

    pub enum ElementNode {
        Track {
            id: u64,
            connection: Option<u64>,
            location: Location,
        },
        Point {
            id: u64,
            reverse: u64,
            location: Location,
        },
        DSPoint {
            id: u64,
            reverse_up: u64,
            reverse_down: u64,
            location: Location,
        },
        SSPoint {
            id: u64,
            reverse: u64,
            location: Location,
        },
        Crossing {
            id: u64,
            location: Location,
        },
        Derailer {
            id: u64,
            location: Location,
        },
        NonBlockingCrossing {
            id: u64,
            location: Location,
        },
    }

    impl ElementNode {
        fn set_location(&mut self, location: Location) {
            todo!()
        }
    }
}
