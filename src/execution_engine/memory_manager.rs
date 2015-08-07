// Copyright 2015 Jauhien Piatlicki.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// LLVM MCJIT Memory Manager
// LLVM-C header ExecutionEngine.h

use std;

use libc::{c_char, c_uint, c_void};

use llvm_sys::prelude::*;
use llvm_sys::execution_engine::*;

use ::LLVMRef;

pub trait MCJITMemoryManager : LLVMRef<LLVMMCJITMemoryManagerRef> {
    unsafe fn unown(&mut self);
}

pub trait SimpleMCJITMemoryManagerImpl {
    fn allocate_code_section(&mut self, size: usize, alignment: u32, section_id: u32, section_name: &str) -> *mut u8;
    fn allocate_data_section(&mut self, size: usize, alignment: u32, section_id: u32, section_name: &str, is_read_only: bool) -> *mut u8;
    fn finalize_memory(&mut self) -> Result<(), String>;
    fn destroy(&mut self);
}

pub struct SimpleMCJITMemoryManager<T: SimpleMCJITMemoryManagerImpl> {
    memory_manager: LLVMMCJITMemoryManagerRef,
    _implementation: Box<T>,
    owned: bool
}

impl<T: SimpleMCJITMemoryManagerImpl> SimpleMCJITMemoryManager<T> {
    pub fn new(mut implementation: T) -> SimpleMCJITMemoryManager<T> {

        extern fn allocate_code_section_callback<T: SimpleMCJITMemoryManagerImpl>(opaque: *mut c_void, size: usize, alignment: c_uint, section_id: c_uint, section_name: *const c_char) -> *mut u8 {
            let implementation = opaque as *mut T;
            unsafe {
                let cstr_buf = std::ffi::CStr::from_ptr(section_name);
                let section_name = std::str::from_utf8_unchecked(cstr_buf.to_bytes());
                (*implementation).allocate_code_section(size, alignment, section_id, section_name)
            }
        }

        extern fn allocate_data_section_callback<T: SimpleMCJITMemoryManagerImpl>(opaque: *mut c_void, size: usize, alignment: c_uint, section_id: c_uint, section_name: *const c_char, is_read_only: LLVMBool) -> *mut u8 {
            let implementation = opaque as *mut T;
            unsafe {
                let cstr_buf = std::ffi::CStr::from_ptr(section_name);
                let section_name = std::str::from_utf8_unchecked(cstr_buf.to_bytes());
                (*implementation).allocate_data_section(size, alignment, section_id, section_name, is_read_only > 0)
            }
        }

        extern fn finalize_memory_callback<T: SimpleMCJITMemoryManagerImpl>(opaque: *mut c_void, error_msg: *mut *mut c_char) -> LLVMBool {
            let implementation = opaque as *mut T;
            match unsafe {(*implementation).finalize_memory()} {
                Ok(_) => 0,
                Err(error) => {
                    let error = std::ffi::CString::new(error).unwrap();
                    unsafe {
                        *error_msg = error.as_ptr() as *mut c_char;
                    }
                    std::mem::forget(error);
                    1
                }
            }
        }

        extern fn destroy_callback<T: SimpleMCJITMemoryManagerImpl>(opaque: *mut c_void) {
            let implementation = opaque as *mut T;
            unsafe {
                (*implementation).destroy();
            }
        }

        let memory_manager = unsafe {
            LLVMCreateSimpleMCJITMemoryManager(&mut implementation as *mut _ as *mut c_void,
                                               allocate_code_section_callback::<T>,
                                               allocate_data_section_callback::<T>,
                                               finalize_memory_callback::<T>,
                                               destroy_callback::<T>)
        };

        SimpleMCJITMemoryManager {
            memory_manager: memory_manager,
            _implementation: Box::new(implementation),
            owned: true
        }
    }
}

impl<T:SimpleMCJITMemoryManagerImpl> LLVMRef<LLVMMCJITMemoryManagerRef> for SimpleMCJITMemoryManager<T> {
    fn to_ref(&self) -> LLVMMCJITMemoryManagerRef {
        self.memory_manager
    }
}

impl<T:SimpleMCJITMemoryManagerImpl> MCJITMemoryManager for SimpleMCJITMemoryManager<T> {
    unsafe fn unown(&mut self) {
        self.owned = false
    }
}

impl<T:SimpleMCJITMemoryManagerImpl> Drop for SimpleMCJITMemoryManager<T> {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMDisposeMCJITMemoryManager(self.memory_manager)
            }
        }
    }
}
