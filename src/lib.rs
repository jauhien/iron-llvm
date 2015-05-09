// Copyright 2015 Jauhien Piatlicki.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

#![feature(convert)]
#![feature(libc)]

#![feature(log_syntax)]
#![feature(trace_macros)]

extern crate libc;
extern crate llvm_sys;

use std::io;
use std::io::Write;

use libc::c_uint;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use core::*;
use core::types::{Type, IntType, IntTypeCtor, FunctionType, FunctionTypeCtor, StructType, StructTypeCtor};

macro_rules! new_ref_type(
    ($ref_type:ident for $llvm_ref_type:ident implementing $($base_trait:ty),+) => (
        pub enum $ref_type {
            Rf($llvm_ref_type)
        }

        impl LLVMRef<$llvm_ref_type> for $ref_type {
            fn to_ref(&self) -> $llvm_ref_type {
                match *self {
                    $ref_type::Rf(rf) => rf
                }
            }
        }

        impl LLVMRefCtor<$llvm_ref_type> for $ref_type {
            unsafe fn from_ref(rf: $llvm_ref_type) -> $ref_type {
                $ref_type::Rf(rf)
            }
        }

        $(
            impl $base_trait for $ref_type {}
        )*
        )
);

pub mod core;

pub trait LLVMRef<Ref> {
    fn to_ref(&self) -> Ref;
}

pub trait LLVMRefCtor<Ref> : Sized {
    unsafe fn from_ref(rf: Ref) -> Self;
}

#[test]
fn it_works() {
    let c1 = context::Context::new();

    let c1_ref = c1.to_ref();

    let gc1 = context::Context::get_global();
    let gc1_ref = gc1.to_ref();

    let gc = unsafe {
        LLVMGetGlobalContext()
    };

    assert!(gc == gc1_ref);
    assert!(c1_ref != gc);

    let ty = unsafe {
        LLVMInt64TypeInContext(gc1.to_ref())
    };

   assert!(ty.get_context().to_ref() == gc1_ref);

    let mut stderr = io::stderr();

    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "Testing Type").unwrap();
    //writeln!(&mut stderr, "kind: {:?}", ty.get_kind()).unwrap();
    writeln!(&mut stderr, "is sized: {:?}", ty.is_sized()).unwrap();
    write!(&mut stderr, "dump: ").unwrap();
    ty.dump();
    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "string rep: {:?}", ty.print_to_string()).unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "").unwrap();

    let int10 = types::IntTypeRef::get_int(10);
    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "Testing int10").unwrap();
    writeln!(&mut stderr, "type width: {:?}", int10.get_width()).unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "").unwrap();

    let mut args = vec![ty.to_ref(), int10.to_ref()];
    let func = types::FunctionTypeRef::get(&ty, args.as_mut_slice(), false);

    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "Testing function type").unwrap();
    writeln!(&mut stderr, "string rep: {:?}", func.print_to_string()).unwrap();
    writeln!(&mut stderr, "is var arg: {:?}", func.is_var_arg()).unwrap();
    writeln!(&mut stderr, "return type: {:?}", func.get_return_type().print_to_string()).unwrap();
    writeln!(&mut stderr, "number of parameters: {:?}", func.count_param_types()).unwrap();

    let params = func.get_param_types();
    for param in params {
        writeln!(&mut stderr, "  param type: {:?}", param.print_to_string()).unwrap();
    }

    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "").unwrap();

    let struct1 = types::StructTypeRef::new_named(&gc1, "testing");
    writeln!(&mut stderr, "").unwrap();
    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "Testing struct type").unwrap();
    writeln!(&mut stderr, "string rep: {:?}", struct1.print_to_string()).unwrap();
    writeln!(&mut stderr, "name: {:?}", struct1.get_name()).unwrap();
    writeln!(&mut stderr, "is opaque: {:?}", struct1.is_opaque()).unwrap();
    writeln!(&mut stderr, "setting body...").unwrap();

    struct1.set_body(args.as_mut_slice(), false);

    writeln!(&mut stderr, "string rep: {:?}", struct1.print_to_string()).unwrap();
    writeln!(&mut stderr, "is opaque: {:?}", struct1.is_opaque()).unwrap();
    writeln!(&mut stderr, "count of elements: {:?}", struct1.count_element_types()).unwrap();

    let elements = struct1.get_element_types();
    for element in elements {
        writeln!(&mut stderr, "  element type: {:?}", element.print_to_string()).unwrap();
    }

    writeln!(&mut stderr, "========").unwrap();
    writeln!(&mut stderr, "").unwrap();
}
