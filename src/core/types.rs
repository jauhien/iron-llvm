// Copyright 2015 Jauhien Piatlicki.
//
// Copyright 2012-2015 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Core LLVM: Type hierarchy
// LLVM-C header Core.h

use std;

use core;
use core::{context, TypeKind, TypeRef};
use self::ffi::*;

pub trait Type {
    fn get_ref(&self) -> TypeRef;

    fn get_kind(&self) -> TypeKind {
        unsafe {
            LLVMGetTypeKind(self.get_ref())
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(self.get_ref()) > 0
        }
    }

    fn get_context(&self) -> context::Context {
        unsafe {
            let ctx = LLVMGetTypeContext(self.get_ref());
            context::Context::from_ref(ctx)
        }
    }

    fn dump(&self) {
        unsafe {
            LLVMDumpType(self.get_ref());
        }
    }

    fn print_to_string(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMPrintTypeToString(self.get_ref()))
        };
        let result = String::from_utf8_lossy(buf.to_bytes()).into_owned();
        unsafe { core::ffi::LLVMDisposeMessage(buf.as_ptr()); }
        result
    }
}

impl Type for TypeRef {
    fn get_ref(&self) -> TypeRef {
        *self
    }
}

pub mod ffi {
    use ::Bool;
    use core::*;
    use libc::{c_char, c_uint};

    #[link(name = "LLVMCore")]
    extern {
        pub fn LLVMGetTypeKind(Ty: TypeRef) -> TypeKind;

        /**
         * Whether the type has a known size.
         *
         * Things that don't have a size are abstract types, labels, and void.a
         */
        pub fn LLVMTypeIsSized(Ty: TypeRef) -> Bool;

        /**
         * Obtain the context to which this type instance is associated.
         */
        pub fn LLVMGetTypeContext(Ty: TypeRef) -> ContextRef;

        /**
         * Dump a representation of a type to stderr.
         */
        pub fn LLVMDumpType(Val: TypeRef);

        /**
         * Return a string representation of the type. Use
         * LLVMDisposeMessage to free the string.
         */
        pub fn LLVMPrintTypeToString(Val: TypeRef) -> *const c_char;

        /**
         * Obtain an integer type from a context with specified bit width.
         */
        pub fn LLVMInt1TypeInContext(C: ContextRef) -> TypeRef;
        pub fn LLVMInt8TypeInContext(C: ContextRef) -> TypeRef;
        pub fn LLVMInt16TypeInContext(C: ContextRef) -> TypeRef;
        pub fn LLVMInt32TypeInContext(C: ContextRef) -> TypeRef;
        pub fn LLVMInt64TypeInContext(C: ContextRef) -> TypeRef;
        pub fn LLVMIntTypeInContext(C: ContextRef, NumBits: c_uint) -> TypeRef;
    }
}
