// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # State

use std::num::Wrapping;
use std::collections::VecDeque;
use machine::Machine;

type Tape = VecDeque<Wrapping<u8>>;

/// Holds the state of the interpreter
pub struct State {
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

    pub fn get_tape(&self) -> Tape {
        self.tape.clone()
    }
}

impl Machine for State {
    fn left(&mut self, val: u8) -> &mut Self {
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

    fn right(&mut self, val: u8) -> &mut Self {
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

    fn increase(&mut self, val: u8) -> &mut Self {
        if let Some(elem) = self.tape.get_mut(self.pos) {
            *elem += Wrapping(val);
        };
        self
    }

    fn decrease(&mut self, val: u8) -> &mut Self {
        if let Some(elem) = self.tape.get_mut(self.pos) {
            *elem -= Wrapping(val);
        };
        self
    }

    fn get_val(&self) -> u8 {
        self.tape[self.pos].0
    }

    fn output(&self) {
        print!("{}", self.get_val() as char);
    }

    fn input(&mut self, val: u8) {
        self.tape[self.pos] = Wrapping(val);
    }
}

impl Default for State {
    /// Default
    fn default() -> Self {
        State::new()
    }
}
