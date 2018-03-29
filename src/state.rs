// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # State

use std::num::Wrapping;
use std::collections::VecDeque;

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

    pub fn get_tape(&self) -> Tape {
        self.tape.clone()
    }
}

impl Default for State {
    /// Default
    fn default() -> Self {
        State::new()
    }
}
