// Copyright 2015 Jauhien Piatlicki.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

#![feature(io)]
#![feature(libc)]
#![feature(std_misc)]

extern crate libc;
#[macro_use] #[no_link] extern crate rustc_bitflags;

use std::io;
use std::io::Write;

use libc::c_uint;

use core::*;
use core::types::{Type, IntType};

pub mod core;

mod llvmdeps;

pub type Bool = c_uint;
pub const True: Bool = 1 as Bool;
pub const False: Bool = 0 as Bool;

#[test]
fn it_works() {
    let c1 = context::Context::new();

    let c1_ref = c1.get_ref();

    let gc1 = context::Context::get_global();
    let gc1_ref = gc1.get_ref();

    let gc = unsafe {
        context::ffi::LLVMGetGlobalContext()
    };

    assert!(gc == gc1_ref);
    assert!(c1_ref != gc);

    let ty = unsafe {
        types::ffi::LLVMInt64TypeInContext(gc1.get_ref())
    };

    assert!(ty.get_context().get_ref() == gc1_ref);

    let mut stderr = io::stderr();

    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "Testing Type").unwrap();
    writeln!(&mut stderr, "kind: {:?}", ty.get_kind()).unwrap();
    writeln!(&mut stderr, "is sized: {:?}", ty.is_sized()).unwrap();
    write!(&mut stderr, "dump: ").unwrap();
    ty.dump();
    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "string rep: {:?}", ty.print_to_string()).unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "").unwrap();

    let int10 = types::IntTypeRef::int_type_in_context(c1, 10);
    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "Testing int10").unwrap();
    writeln!(&mut stderr, "type width: {:?}", int10.get_width()).unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "").unwrap();
}
