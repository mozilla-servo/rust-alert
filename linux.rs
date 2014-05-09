// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use AlertMethods;
use std::io;
use std::io::BufferedReader;

/// An alert.
pub struct Alert {
    url: ~str,
}

impl AlertMethods for Alert {
    fn new(_: &str) -> Alert {
        // TODO: Use a glfw window for prompt
        Alert { url: "".to_owned() }
    }

    fn add_prompt(&mut self) {
        println!("URL: ");
        self.url = match BufferedReader::new(io::stdin()).read_line() {
            Ok(res) => res,
            _ => fail!("Could not read URL from stdin"),
            
        }
    }

    fn run(&self) {
    }

    fn prompt_value(&self) -> ~str {
        self.url.clone()
    }
}

