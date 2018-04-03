// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Machine

/// The `Machine` trait needs to be implemented by every interpreter/(cross)compiler
pub trait Machine {
    /// Move left
    fn left(&mut self, val: u8) -> &mut Self;

    /// Move right
    fn right(&mut self, val: u8) -> &mut Self;

    /// Increase
    fn increase(&mut self, val: u8) -> &mut Self;

    /// Increase
    fn decrease(&mut self, val: u8) -> &mut Self;

    /// Output
    fn output(&self);

    /// Set value
    fn input(&mut self, val: u8);

    /// Get value
    fn get_val(&self) -> u8;
}
