// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Lexer

type TokenPosition = usize;
/// This type represents a vector of tuples, where each tuple holds a token and the position of the
/// token within a brainfuck program.
pub type TokenStream = Vec<(TokenPosition, Token)>;

/// Defines all possible Tokens allowed in a brainfuck program
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    /// '+'
    Increase,
    /// '-'
    Decrease,
    /// '<'
    MoveLeft,
    /// '>'
    MoveRight,
    /// '['
    LoopBegin(Option<usize>),
    /// ']'
    LoopEnd(Option<usize>),
    /// ','
    Input,
    /// '.'
    Output,
    /// anything else
    Comment,
}

/// The lexer takes a brainfuck program and returns a stream of tokens
pub fn lexer(prog: &str) -> TokenStream {
    prog.chars()
        .enumerate()
        .map(|(i, x)| {
            (
                i,
                match x {
                    '+' => Token::Increase,
                    '-' => Token::Decrease,
                    '<' => Token::MoveLeft,
                    '>' => Token::MoveRight,
                    ',' => Token::Input,
                    '.' => Token::Output,
                    '[' => Token::LoopBegin(None),
                    ']' => Token::LoopEnd(None),
                    _ => Token::Comment,
                },
            )
        })
        .collect()
}
