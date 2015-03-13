// Copyright 2015 Jauhien Piatlicki.
//
// Copyright 2012-2015 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Core LLVM: Value hierarchy
// LLVM-C header Core.h

use std;

use core;
use core::{TypeRef, ValueRef, UseRef};
use self::ffi::*;

pub trait Value {
    fn get_ref(&self) -> ValueRef;

    fn type_of(&self) -> TypeRef {
        unsafe {
            LLVMTypeOf(self.get_ref())
        }
    }

    fn get_name(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMGetValueName(self.get_ref()))
        };
        let result = String::from_utf8_lossy(buf.to_bytes()).into_owned();
        result
    }

    fn set_name(&self, name: &str) {
        unsafe {
            LLVMSetValueName(self.get_ref(), name.as_ptr() as *const i8)
        }
    }

    fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.get_ref())
        }
    }

    fn print_to_string(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMPrintValueToString(self.get_ref()))
        };
        let result = String::from_utf8_lossy(buf.to_bytes()).into_owned();
        unsafe { core::ffi::LLVMDisposeMessage(buf.as_ptr()); }
        result
    }

    fn replace_all_uses_with(&self, new_val: &Value) {
        unsafe {
            LLVMReplaceAllUsesWith(self.get_ref(), new_val.get_ref())
        }
    }

    fn is_constant(&self) -> bool {
        unsafe {
            LLVMIsConstant(self.get_ref()) > 0
        }
    }

    fn is_undef(&self) -> bool {
        unsafe {
            LLVMIsUndef(self.get_ref()) > 0
        }
    }

    fn use_iter(&self) -> UseIter {
        let first = unsafe {
            LLVMGetFirstUse(self.get_ref())
        };

        let current = if first.is_null() {
            None
        } else {
            Some(first)
        };

        UseIter{current: current}
    }
}

impl Value for ValueRef {
    fn get_ref(&self) -> ValueRef {
        *self
    }
}

pub trait Use {
    fn get_ref(&self) -> UseRef;

    fn get_user(&self) -> UserRef {
        unsafe {
            UserRef::Ref(LLVMGetUser(self.get_ref()))
        }
    }

    fn get_used_value(&self) -> ValueRef {
        unsafe {
            LLVMGetUsedValue(self.get_ref())
        }
    }
}

impl Use for UseRef {
    fn get_ref(&self) -> UseRef {
        *self
    }
}

pub struct UseIter {
    current: Option<UseRef>
}

impl Iterator for UseIter {
    type Item = UseRef;

    fn next(&mut self) -> Option<UseRef> {
        match self.current {
            Some(cur) => {
                let next = unsafe {
                    LLVMGetNextUse(cur)
                };
                self.current = if next.is_null() {
                    None
                } else {
                    Some(next)
                };

                self.current
            },
            None => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        return (0, None)
    }
}


// TODO: implement iterator and some better indexing for User

pub enum UserRef {
    Ref(ValueRef)
}

impl Value for UserRef {
    fn get_ref(&self) -> ValueRef {
        match *self {
            UserRef::Ref(rf) => rf
        }
    }
}

pub trait User : Value {
    fn get_operand(&self, index: u32) -> ValueRef {
        unsafe {
            LLVMGetOperand(self.get_ref(), index)
        }
    }

    fn get_operand_use(&self, index: u32) -> UseRef {
        unsafe {
            LLVMGetOperandUse(self.get_ref(), index)
        }
    }

    fn set_operand(&self, index: u32, op: &Value) {
        unsafe {
            LLVMSetOperand(self.get_ref(), index, op.get_ref())
        }
    }

    fn get_num_operands(&self) -> i32 {
        unsafe {
            LLVMGetNumOperands(self.get_ref())
        }
    }
}

impl User for UserRef {}

pub mod ffi {
    use ::Bool;
    use core::*;
    use libc::{c_char, c_int, c_uint};

    #[link(name = "LLVMCore")]
    extern {
        /* Operations on all values */

        /**
         * Obtain the type of a value.
         */
        pub fn LLVMTypeOf(Val: ValueRef) -> TypeRef;

        /**
         * Obtain the string name of a value.
         */
        pub fn LLVMGetValueName(Val: ValueRef) -> *const c_char;

        /**
         * Set the string name of a value.
         */
        pub fn LLVMSetValueName(Val: ValueRef, Name: *const c_char);

        /**
         * Dump a representation of a value to stderr.
         */
        pub fn LLVMDumpValue(Val: ValueRef);

        /**
         * Return a string representation of the value. Use
         * LLVMDisposeMessage to free the string.
         */
        pub fn LLVMPrintValueToString(Val: ValueRef) -> *const c_char;

        /**
         * Replace all uses of a value with another one.
         */
        pub fn LLVMReplaceAllUsesWith(OldVal: ValueRef, NewVal: ValueRef);

        /**
         * Determine whether the specified constant instance is constant.
         */
        pub fn LLVMIsConstant(Val: ValueRef) -> Bool;

        /**
         * Determine whether a value instance is undefined.
         */
        pub fn LLVMIsUndef(Val: ValueRef) -> Bool;


        /* Operations on Uses */

        /**
         * Obtain the first use of a value.
         *
         * Uses are obtained in an iterator fashion. First, call this function
         * to obtain a reference to the first use. Then, call LLVMGetNextUse()
         * on that instance and all subsequently obtained instances until
         * LLVMGetNextUse() returns NULL.
         */
        pub fn LLVMGetFirstUse(Val: ValueRef) -> UseRef;

        /**
         * Obtain the next use of a value.
         *
         * This effectively advances the iterator. It returns NULL if you are on
         * the final use and no more are available.
         */
        pub fn LLVMGetNextUse(U: UseRef) -> UseRef;

        /**
         * Obtain the user value for a user.
         *
         * The returned value corresponds to a llvm::User type.
         */
        pub fn LLVMGetUser(U: UseRef) -> ValueRef;

        /**
         * Obtain the value this use corresponds to.
         */
        pub fn LLVMGetUsedValue(U: UseRef) -> ValueRef;


        /* Operations on Users */

        /**
         * Obtain an operand at a specific index in a llvm::User value.
         */
        pub fn LLVMGetOperand(Val: ValueRef, Index: c_uint) -> ValueRef;

        /**
         * Obtain the use of an operand at a specific index in a llvm::User value.
         */
        pub fn LLVMGetOperandUse(Val: ValueRef, Index: c_uint) -> UseRef;

        /**
         * Set an operand at a specific index in a llvm::User value.
         */
        pub fn LLVMSetOperand(Val: ValueRef, Index: c_uint, Op: ValueRef);

        /**
         * Obtain the number of operands in a llvm::User value.
         */
        pub fn LLVMGetNumOperands(Val: ValueRef) -> c_int;
    }
}
