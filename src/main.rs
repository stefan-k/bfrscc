// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A basic, certainly not optimized, Brainfuck interpreter.

use std::collections::VecDeque;
use std::num::Wrapping;

type TokenPosition = usize;
type TokenStream = Vec<(TokenPosition, Token)>;
type InstructionStream = Vec<Instruction>;
type Tape = VecDeque<Wrapping<u8>>;

/// Holds the state of the interpreter
struct State {
    /// Current position in the buffer
    pos: usize,
    /// Tape
    tape: Tape,
}

impl State {
    /// Constructor
    pub fn new() -> Self {
        let mut tape = VecDeque::new();
        tape.push_back(Wrapping(0));
        State { pos: 0, tape }
    }

    pub fn left(&mut self, val: u8) -> &mut Self {
        match self.pos {
            // We are already at the beginning of the tape, so we will just push to the
            // front. Decreasing `state.pos` is not necessary.
            0 => for _ in 0..val {
                self.tape.push_front(Wrapping(0));
            },
            // Just move the pointer to the left
            _ => self.pos -= val as usize,
        };
        self
    }

    pub fn right(&mut self, val: u8) -> &mut Self {
        for _ in 0..val {
            self.pos += 1;
            match self.tape.get(self.pos) {
                // The tape is not empty at the current position.
                Some(_) => {}
                // We have exceeded the tape and need to add another element
                None => self.tape.push_back(Wrapping(0)),
            };
        }
        self
    }

    pub fn increase(&mut self, val: u8) -> &mut Self {
        if let Some(elem) = self.tape.get_mut(self.pos) {
            *elem += Wrapping(val);
        };
        self
    }

    pub fn decrease(&mut self, val: u8) -> &mut Self {
        if let Some(elem) = self.tape.get_mut(self.pos) {
            *elem -= Wrapping(val);
        };
        self
    }

    pub fn get_val(&self) -> u8 {
        self.tape.get(self.pos).unwrap().0
    }
}

impl Default for State {
    /// Default
    fn default() -> Self {
        State::new()
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Token {
    Increase,
    Decrease,
    MoveLeft,
    MoveRight,
    LoopBegin(Option<usize>),
    LoopEnd(Option<usize>),
    Input,
    Output,
    Comment,
}

#[derive(Debug, Clone)]
struct Instruction {
    position: usize,
    token: Token,
    multiplier: u8,
}

impl Instruction {
    pub fn new(position: usize, token: Token, multiplier: u8) -> Self {
        Instruction {
            position,
            token,
            multiplier,
        }
    }
}

fn get_instruction_idx(stream: &InstructionStream, position: usize) -> Option<usize> {
    for (idx, elem) in stream.iter().enumerate() {
        if elem.position == position {
            return Some(idx);
        }
    }
    None
}

fn lexer(prog: &str) -> TokenStream {
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

fn parser(prog: TokenStream) -> InstructionStream {
    // get rid of everything that is not an instruction
    let mut p: TokenStream = prog.into_iter()
        .filter(|&(_, ref x)| *x != Token::Comment)
        .collect();

    // Deal with the loops
    let p2 = p.clone();
    let loops = p2.into_iter().filter(|&(_, ref x)| match *x {
        Token::LoopBegin(_) => true,
        Token::LoopEnd(_) => true,
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

fn main() {
    // Hello World
    // let prog = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".as_bytes();
    let prog = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    // let prog = "++[->+<]".as_bytes();
    let prog_lex = lexer(prog);
    // println!("{:?}", bla);
    let prog = parser(prog_lex);
    println!("{:?}", prog);

    // The buffer

    // State holds the position of the pointer
    let mut state = State::new();

    // length of the program.
    let plen = prog.len();
    let mut idx = 0;
    loop {
        // Get the current instruction.
        match prog.get(idx).unwrap() {
            // Move right
            &Instruction {
                token: Token::MoveRight,
                multiplier: m,
                ..
            } => {
                state.right(m);
            }
            // Move left
            &Instruction {
                token: Token::MoveLeft,
                multiplier: m,
                ..
            } => {
                state.left(m);
            }
            // Increase the value at the current tape position. Allow for buffer overflows!
            &Instruction {
                token: Token::Increase,
                multiplier: m,
                ..
            } => {
                state.increase(m);
            }
            // Decrease the value at the current tape position. Allow for buffer overflows!
            &Instruction {
                token: Token::Decrease,
                multiplier: m,
                ..
            } => {
                state.decrease(m);
            }
            // Print the `char` at the current tape position.
            &Instruction {
                token: Token::Output,
                ..
            } => print!("{}", state.get_val() as char),
            // We found a `[` which indicates the start of a loop
            &Instruction {
                token: Token::LoopBegin(Some(lb)),
                ..
            } => {
                if state.get_val() == 0 {
                    // Value at current tape is `0`, therefore we jump to the position after the
                    // matching `]`. Otherwise do nothing (means moving on).
                    if let Some(x) = get_instruction_idx(&prog, lb) {
                        idx = x;
                    } else {
                        panic!("No matching ] found. This cannot happen.");
                    }
                }
            }
            // We found a `]` which indicates the end of a loop
            &Instruction {
                token: Token::LoopEnd(Some(le)),
                ..
            } => {
                if state.get_val() != 0 {
                    // If the value in the tape at the current position is nonzero, we move to
                    // the matching `[`. Otherwise do nothing (means moving on).
                    if let Some(x) = get_instruction_idx(&prog, le) {
                        idx = x;
                    } else {
                        panic!("No matching [ found. This cannot happen.");
                    }
                }
            }
            // Match any other Token...
            _ => {}
        };
        // Move to the next instruction, break if end of program is reached.
        idx += 1;
        if idx >= plen {
            break;
        }
    }
    // Print the final tape
    println!("Tape: {:?}", state.tape);
}
