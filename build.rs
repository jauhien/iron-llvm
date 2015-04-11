// Copyright 2015 Jauhien Piatlicki.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

fn main() {
    Command::new("python2").arg("src/etc/mklldeps.py")
        .arg("src/llvmdeps.rs")
        .arg("core")
        .arg("llvm-config")
        .status()
        .unwrap();
}
