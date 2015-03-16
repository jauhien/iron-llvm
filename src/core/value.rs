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

use ::{LLVMRef, LLVMRefCtor};
use core;
use core::{TypeRef, ValueRef, UseRef};
use self::ffi::*;

pub trait ValueCtor : LLVMRefCtor<ValueRef> {}

pub trait Value : LLVMRef<ValueRef> {
    fn get_type(&self) -> TypeRef {
        unsafe {
            LLVMTypeOf(self.to_ref())
        }
    }

    fn get_name(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMGetValueName(self.to_ref()))
        };
        let result = String::from_utf8_lossy(buf.to_bytes()).into_owned();
        result
    }

    fn set_name(&self, name: &str) {
        unsafe {
            LLVMSetValueName(self.to_ref(), name.as_ptr() as *const i8)
        }
    }

    fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.to_ref())
        }
    }

    fn print_to_string(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMPrintValueToString(self.to_ref()))
        };
        let result = String::from_utf8_lossy(buf.to_bytes()).into_owned();
        unsafe { core::ffi::LLVMDisposeMessage(buf.as_ptr()); }
        result
    }

    fn replace_all_uses_with(&self, new_val: &Value) {
        unsafe {
            LLVMReplaceAllUsesWith(self.to_ref(), new_val.to_ref())
        }
    }

    fn is_constant(&self) -> bool {
        unsafe {
            LLVMIsConstant(self.to_ref()) > 0
        }
    }

    fn is_undef(&self) -> bool {
        unsafe {
            LLVMIsUndef(self.to_ref()) > 0
        }
    }

    fn use_iter(&self) -> UseIter {
        let first = unsafe {
            LLVMGetFirstUse(self.to_ref())
        };

        let current = if first.is_null() {
            None
        } else {
            Some(first)
        };

        UseIter{current: current}
    }
}

impl LLVMRef<ValueRef> for ValueRef {
    fn to_ref(&self) -> ValueRef {
        *self
    }
}

impl LLVMRefCtor<ValueRef> for ValueRef {
    unsafe fn from_ref(rf: ValueRef) -> ValueRef {
        rf
    }
}

impl Value for ValueRef {}
impl ValueCtor for ValueRef {}

pub trait UseCtor : LLVMRefCtor<UseRef> {}

pub trait Use : LLVMRef<UseRef> {
    fn get_user(&self) -> UserRef {
        unsafe {
            UserRef::from_ref(LLVMGetUser(self.to_ref()))
        }
    }

    fn get_used_value(&self) -> ValueRef {
        unsafe {
            LLVMGetUsedValue(self.to_ref())
        }
    }
}

impl LLVMRef<UseRef> for UseRef {
    fn to_ref(&self) -> UseRef {
        *self
    }
}

impl LLVMRefCtor<UseRef> for UseRef {
    unsafe fn from_ref(rf: UseRef) -> UseRef {
        rf
    }
}

impl Use for UseRef {}
impl UseCtor for UseRef {}

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

pub trait User : Value {
    fn get_operand(&self, index: u32) -> ValueRef {
        unsafe {
            LLVMGetOperand(self.to_ref(), index)
        }
    }

    fn get_operand_use(&self, index: u32) -> UseRef {
        unsafe {
            UseRef::from_ref(LLVMGetOperandUse(self.to_ref(), index))
        }
    }

    fn set_operand(&self, index: u32, op: &Value) {
        unsafe {
            LLVMSetOperand(self.to_ref(), index, op.to_ref())
        }
    }

    fn get_num_operands(&self) -> i32 {
        unsafe {
            LLVMGetNumOperands(self.to_ref())
        }
    }
}

new_ref_type!(UserRef for ValueRef implementing Value);

pub trait ConstCtor<Ty: core::types::Type + ?Sized> : ValueCtor {
    fn get_null(ty: &Ty) -> Self {
        unsafe {
            Self::from_ref(LLVMConstNull(ty.to_ref()))
        }
    }

    fn get_undef(ty: &Ty) -> Self {
        unsafe {
            Self::from_ref(LLVMGetUndef(ty.to_ref()))
        }
    }

    fn get_pointer_null(ty: &Ty) -> Self {
        unsafe {
            Self::from_ref(LLVMConstPointerNull(ty.to_ref()))
        }
    }
}

pub trait Const : User {
    fn is_null(&self) -> bool {
        unsafe {
            LLVMIsNull(self.to_ref()) > 0
        }
    }
}

new_ref_type!(ConstRef for ValueRef
              implementing
              Value,
              User,
              Const,
              ValueCtor,
              ConstCtor<core::types::Type>
              );


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


        /* Operations on constants of any type */

        /**
         * Obtain a constant value referring to the null instance of a type.
         */
        pub fn LLVMConstNull(Ty: TypeRef) -> ValueRef;

        /**
         * Obtain a constant value referring to the instance of a type
         * consisting of all ones.
         *
         * This is only valid for integer types.
         */
        pub fn LLVMConstAllOnes(Ty: TypeRef) -> ValueRef;

        /**
         * Obtain a constant value referring to an undefined value of a type.
         */
        pub fn LLVMGetUndef(Ty: TypeRef) -> ValueRef;

        /**
         * Determine whether a value instance is null.
         */
        pub fn LLVMIsNull(Val: ValueRef) -> Bool;

        /**
         * Obtain a constant that is a constant pointer pointing to NULL for a
         * specified type.
         */
        pub fn LLVMConstPointerNull(Ty: TypeRef) -> ValueRef;
    }
}
