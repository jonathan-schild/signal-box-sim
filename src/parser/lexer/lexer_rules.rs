/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use crate::{
    Error,
    parser::{Token, lexer::LexerState},
};

pub fn scan_line(state: &mut LexerState) -> Result<(), Error> {
    while let Some(c) = state.peek() {
        let c = *c; // TODO make it clean

        // Skip whitespace
        if c.is_whitespace() {
            scan_keyword_or_identifier(state)?;
            skip_whitespace(state);
        } else
        // Scan punctuation
        if state.is_empty() && c.is_ascii_punctuation() {
            scan_punctuation(state)?;
        } else {
            state.take();
        }
    }

    scan_keyword_or_identifier(state)?;

    Ok(())
}

fn skip_whitespace(state: &mut LexerState) {
    while let Some(c) = state.peek() {
        if !c.is_whitespace() {
            break;
        }
        state.skip();
    }
}

fn scan_punctuation(state: &mut LexerState) -> Result<(), Error> {
    let Some(c) = state.peek() else { todo!() };
    let c = *c; // TODO clean that

    match c {
        ',' => {
            state.skip();
            state.accept(Token::COMMA);
        }
        ';' => {
            state.skip();
            state.accept(Token::SEMICOLON);
        }
        '-' => {
            state.skip();
            state.accept(Token::DASH);
        }
        '[' => {
            state.skip();
            state.accept(Token::LBRACKET);
        }
        ']' => {
            state.skip();
            state.accept(Token::RBRACKET);
        }
        _ => {
            return Err(Error::UnexpectedSymbol(
                c,
                r#"",", ";", "-", "[" or "]""#.to_string(),
            ));
        }
    };

    Ok(())
}

fn scan_keyword_or_identifier(state: &mut LexerState) -> Result<(), Error> {
    let token = match state.buffer() {
        "W" => Token::W_POINT,
        "DKW" => Token::DKW_DOUBLE_SLIP_POINT,
        "EKW" => Token::EKW_SINGLE_SLIP_POINT,
        "K" => Token::K_CROSSING,
        "T" => Token::T_TUNNEL,
        "Gs" => Token::GS_DERAILER,
        _ => scan_identifier(state)?,
    };

    state.accept(token);

    Ok(())
}

fn scan_identifier(state: &mut LexerState) -> Result<Token, Error> {
    for c in state.buffer().chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
        } else {
            return Err(Error::InvalidIdentifier(state.buffer().to_string()));
        }
    }
    Ok(Token::IDENTIFIER(state.buffer().to_string()))
}
