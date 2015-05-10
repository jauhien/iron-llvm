// Copyright 2015 Jauhien Piatlicki.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Core LLVM: Module
// LLVM-C header Core.h

use std;

use libc::c_char;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use ::{LLVMRef, LLVMRefCtor};
use core::context;
use core::value::FunctionRef;

pub struct Module {
    module: LLVMModuleRef
}

//TODO: add print to file and metadata methods
impl Module {
    pub fn new(module_id: &str) -> Module {
        unsafe {
            Module {
                module: LLVMModuleCreateWithName(module_id.as_ptr() as *const c_char)
            }
        }
    }

    pub fn new_in_context(module_id: &str, ctx: &context::Context) -> Module {
        unsafe {
            Module {
                module: LLVMModuleCreateWithNameInContext(module_id.as_ptr() as *const c_char, ctx.to_ref())
            }
        }
    }

    pub fn get_data_layout(&self) -> String {
        unsafe {
            let buf = std::ffi::CStr::from_ptr(LLVMGetDataLayout(self.to_ref()));
            String::from_utf8_lossy(buf.to_bytes()).into_owned()
        }
    }

    pub fn set_data_layout(&self, triple: &str) {
        unsafe {
            LLVMSetDataLayout(self.to_ref(), triple.as_ptr() as *const c_char);
        }
    }

    pub fn get_target(&self) -> String {
        unsafe {
            let buf = std::ffi::CStr::from_ptr(LLVMGetTarget(self.to_ref()));
            String::from_utf8_lossy(buf.to_bytes()).into_owned()
        }
    }

    pub fn set_target(&self, triple: &str) {
        unsafe {
            LLVMSetTarget(self.to_ref(), triple.as_ptr() as *const c_char);
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(self.to_ref());
        }
    }

    pub fn print_to_string(&self) -> String {
        unsafe {
            let buf = LLVMPrintModuleToString(self.to_ref());
            let cstr_buf = std::ffi::CStr::from_ptr(buf);
            let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
            LLVMDisposeMessage(buf);
            result
        }
    }

    pub fn set_inline_asm(&self, asm: &str) {
        unsafe {
            LLVMSetModuleInlineAsm(self.to_ref(), asm.as_ptr() as *const c_char);
        }
    }

    pub fn get_context(&self) -> context::Context {
        unsafe {
            let ctx = LLVMGetModuleContext(self.to_ref());
            context::Context::from_ref(ctx)
        }
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<LLVMTypeRef> {
        let ty = unsafe {
            LLVMGetTypeByName(self.to_ref(), name.as_ptr() as *const c_char)
        };
        if !ty.is_null() {
            Some(ty)
        } else {
            None
        }
    }

    pub fn get_function_by_name(&self, name: &str) -> Option<FunctionRef> {
        let function = unsafe {
            LLVMGetNamedFunction(self.to_ref(), name.as_ptr() as *const c_char)
        };
        if !function.is_null() {
            unsafe {
                Some(FunctionRef::from_ref(function))
            }
        } else {
            None
        }
    }

    pub fn function_iter(&self) -> FunctionIter {
        let first = unsafe {
            LLVMGetFirstFunction(self.to_ref())
        };

        let current = if first.is_null() {
            None
        } else {
            unsafe {
                Some(FunctionRef::from_ref(first))
            }
        };

        FunctionIter{current: current}
    }
}

pub struct FunctionIter {
    current: Option<FunctionRef>
}

impl Iterator for FunctionIter {
    type Item = FunctionRef;

    fn next(&mut self) -> Option<FunctionRef> {
        match self.current {
            Some(cur) => {
                let next = unsafe {
                    LLVMGetNextFunction(cur.to_ref())
                };
                self.current = if next.is_null() {
                    None
                } else {
                    unsafe {
                        Some(FunctionRef::from_ref(next))
                    }
                };

                self.current
            },
            None => None
        }
    }
}

impl LLVMRef<LLVMModuleRef> for Module {
    fn to_ref(&self) -> LLVMModuleRef {
        self.module
    }
}

impl Clone for Module {
    fn clone(&self) -> Self {
        unsafe {
            Module {
                module: LLVMCloneModule(self.to_ref())
            }
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.to_ref());
        }
    }
}
