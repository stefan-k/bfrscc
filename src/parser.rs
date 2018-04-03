// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Parser

use lexer::{Token, TokenStream};

/// A vector of `Instruction`s. This may be replace with a more complex data structure in the
/// future.
pub type InstructionStream = Vec<Instruction>;

/// Describes a single Instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Position of the instruction within the program
    pub position: usize,
    /// Kind of instruction
    pub token: Token,
    /// Indicates how often this instruction is repeated
    pub multiplier: u8,
}

impl Instruction {
    /// Constructor
    pub fn new(position: usize, token: Token, multiplier: u8) -> Self {
        Instruction {
            position,
            token,
            multiplier,
        }
    }
}

/// Parses the stream of tokens.
pub fn parser(prog: TokenStream) -> InstructionStream {
    // get rid of everything that is not an instruction
    let mut p: TokenStream = prog.into_iter()
        .filter(|&(_, ref x)| *x != Token::Comment)
        .collect();

    // Deal with the loops
    let p2 = p.clone();
    let loops = p2.into_iter().filter(|&(_, ref x)| match *x {
        Token::LoopBegin(_) | Token::LoopEnd(_) => true,
        _ => false,
    });
    let mut stack = vec![];
    for (idx, instr) in loops {
        match instr {
            Token::LoopBegin(_) => stack.push(idx),
            Token::LoopEnd(_) => {
                let tmp = stack.pop().unwrap();
                p[tmp] = (tmp, Token::LoopBegin(Some(idx)));
                p[idx] = (idx, Token::LoopEnd(Some(tmp)));
            }
            _ => unreachable!(),
        }
    }
    // map to instructions
    p.into_iter()
        .map(|(i, x)| Instruction::new(i, x, 1))
        .collect()
}
