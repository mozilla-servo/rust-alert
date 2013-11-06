// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This is not a typical test harness, because alerts require user input.

extern mod alert = "rust-alert";

#[cfg(target_os="macos")]
extern mod cocoa;

use alert::Alert;
use alert::AlertMethods;

#[start]
fn start(argc: int, argv: **u8) -> int {
    do std::rt::start_on_main_thread(argc, argv) {
        main()
    }
}

fn main() {
    init();

    let mut alert: Alert = AlertMethods::new("All units destroyed.");
    alert.add_prompt();
    alert.run()
}

#[cfg(target_os="macos")]
fn init() {
    use cocoa::appkit;
    let _ = appkit::NSApp();
}

#[cfg(not(target_os="macos"))]
fn init() {
}
