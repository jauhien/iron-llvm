// Copyright 2015 Jauhien Piatlicki.
//
// Copyright 2012-2015 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Core LLVM: Context
// LLVM-C header Core.h

use core::ContextRef;
use self::ffi::*;

pub struct Context {
    context: ContextRef,
    owned: bool
}

impl Context {
    pub fn new() -> Context {
        let context = unsafe {
            LLVMContextCreate()
        };

        Context{
            context: context,
            owned: false
        }
    }

    pub fn get_global() -> Context {
        let context = unsafe {
            LLVMGetGlobalContext()
        };

        Context{
            context: context,
            owned: true
        }
    }

    pub unsafe fn from_ref(context_ref : ContextRef) -> Context {
        Context{
            context: context_ref,
            owned: false
        }
    }

    pub fn get_ref(&self) -> ContextRef {
        self.context
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !self.owned {
            unsafe {
                LLVMContextDispose(self.context);
            }
        }
    }
}

pub mod ffi {
    use core::*;
    use libc::{c_char, c_void, c_uint};

    #[link(name = "LLVMCore")]
    extern {
        /**
         * Create a new context.
         *
         * Every call to this function should be paired with a call to
         * LLVMContextDispose() or the context will leak memory.
         */
        pub fn LLVMContextCreate() -> ContextRef;

        /**
         * Obtain the global context instance.
         */
        pub fn LLVMGetGlobalContext() -> ContextRef;

        /**
         * Set the diagnostic handler for this context.
         */
        pub fn LLVMContextSetDiagnosticHandler(C: ContextRef,
                                               Handler: DiagnosticHandler,
                                               DiagnosticContext: *mut c_void);

        /**
         * Set the yield callback function for this context.
         */
        pub fn LLVMContextSetYieldCallback(C: ContextRef,
                                           Handler: YieldCallback,
                                           OpaqueHandle: *mut c_void);

        /**
         * Destroy a context instance.
         *
         * This should be called for every call to LLVMContextCreate() or memory
         * will be leaked.
         */
        pub fn LLVMContextDispose(C: ContextRef);

        /**
         * Return a string representation of the DiagnosticInfo. Use
         * LLVMDisposeMessage to free the string.
         */
        pub fn LLVMGetDiagInfoDescription(DI: DiagnosticInfoRef) -> *const c_char;

        pub fn LLVMGetDiagInfoSeverity(DI: DiagnosticInfoRef) -> DiagnosticSeverity;

        pub fn LLVMGetMDKindIDInContext(C: ContextRef,
                                        Name: *const c_char,
                                        SLen: c_uint) -> c_uint;

        pub fn LLVMGetMDKindID(Name: *const c_char,
                               SLen: c_uint);
    }
}
