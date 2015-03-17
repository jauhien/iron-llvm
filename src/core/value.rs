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

use libc::{c_char, c_uint, c_ulonglong};

use ::{LLVMRef, LLVMRefCtor};
use core;
use core::{Attribute, CallConv, TypeRef, ValueRef, UseRef};
use core::types::{Type, IntType, RealType};
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

pub trait ConstCtor<Ty: Type + ?Sized> : ValueCtor {
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

pub trait IntConstCtor : ConstCtor<IntType> {
    fn get_all_ones(ty: &IntType) -> Self {
        unsafe {
            Self::from_ref(LLVMConstAllOnes(ty.to_ref()))
        }
    }

    fn get(ty: &IntType, val: u64, sign_extend: bool) -> Self {
        unsafe {
            Self::from_ref(LLVMConstInt(ty.to_ref(), val, sign_extend as ::Bool))
        }
    }

    fn get_arbitrary_precision(ty: &IntType, words: &[u64]) -> Self {
        unsafe {
            Self::from_ref(LLVMConstIntOfArbitraryPrecision(ty.to_ref(),
                                                            words.len() as c_uint,
                                                            words.as_ptr()))
        }
    }

    fn get_from_string(ty: &IntType, text: &str, radix: u8) -> Self {
        unsafe {
            Self::from_ref(LLVMConstIntOfString(ty.to_ref(),
                                                text.as_ptr() as *const c_char,
                                                radix))
        }
    }
}

pub trait IntConst : Const {
    fn get_z_ext_value(&self) -> u64 {
        unsafe {
            LLVMConstIntGetZExtValue(self.to_ref())
        }
    }

    fn get_s_ext_value(&self) -> i64 {
        unsafe {
            LLVMConstIntGetSExtValue(self.to_ref())
        }
    }
}

new_ref_type!(IntConstRef for ValueRef
              implementing
              Value,
              User,
              Const,
              ValueCtor,
              ConstCtor<IntType>,
              IntConstCtor,
              IntConst
              );

pub trait RealConstCtor : ConstCtor<RealType> {
    fn get(ty: &RealType, val: f64) -> Self {
        unsafe {
            Self::from_ref(LLVMConstReal(ty.to_ref(), val))
        }
    }

    fn get_from_string(ty: &RealType, text: &str) -> Self {
        unsafe {
            Self::from_ref(LLVMConstRealOfString(ty.to_ref(), text.as_ptr() as *const c_char))
        }
    }
}

pub trait RealConst : Const {
    fn get_double(&self) -> (f64, bool) {
        let mut info_lost: ::Bool = 0;
        let val = unsafe {
            LLVMConstRealGetDouble(self.to_ref(), &mut info_lost)
        };

        (val, info_lost > 0)
    }
}

new_ref_type!(RealConstRef for ValueRef
              implementing
              Value,
              User,
              Const,
              ValueCtor,
              ConstCtor<RealType>,
              RealConstCtor,
              RealConst
              );

pub trait Function : Const {
    fn get_intrinsic_id(&self) -> u32 {
        unsafe {
            LLVMGetIntrinsicID(self.to_ref())
        }
    }

    fn get_call_conv(&self) -> CallConv {
        unsafe {
            std::mem::transmute(LLVMGetFunctionCallConv(self.to_ref()))
        }
    }

    fn set_call_conv(&self, cc: CallConv) {
        unsafe {
            LLVMSetFunctionCallConv(self.to_ref(), cc as c_uint)
        }
    }

    fn add_attr(&self, attr: Attribute) {
        unsafe {
            LLVMAddFunctionAttr(self.to_ref(), attr.bits() as c_ulonglong)
        }
    }

    fn get_attr(&self) -> Attribute {
        unsafe {
            Attribute::from_bits_truncate(LLVMGetFunctionAttr(self.to_ref()) as u32)
        }
    }

    fn remove_attr(&self, attr: Attribute) {
        unsafe {
            LLVMRemoveFunctionAttr(self.to_ref(), attr.bits() as c_ulonglong)
        }
    }

    fn count_params(&self) -> u32 {
        unsafe {
            LLVMCountParams(self.to_ref())
        }
    }

    fn get_params(&self) -> Vec<ValueRef> {
        let params_count = self.count_params();
        let mut buf : Vec<ValueRef> = Vec::with_capacity(params_count as usize);
        let p = buf.as_mut_ptr();
        unsafe {
            std::mem::forget(buf);
            LLVMGetParams(self.to_ref(), p);
            Vec::from_raw_parts(p, params_count as usize, params_count as usize)
        }
    }

    fn get_param(&self, index: u32) -> ArgumentRef {
        unsafe {
            ArgumentRef::from_ref(LLVMGetParam(self.to_ref(), index))
        }
    }

    fn params_iter(&self) -> ArgsIter {
        ArgsIter{
            current: None,
            func: self.to_ref(),
            pos: 0,
            size: 0
        }
    }
}

new_ref_type!(FunctionRef for ValueRef
              implementing
              Value,
              User,
              Const,
              Function
              );

//TODO Add args back iter

pub trait Argument : Value {
    fn get_parent(&self) -> FunctionRef {
        unsafe {
            FunctionRef::from_ref(LLVMGetParamParent(self.to_ref()))
        }
    }

    fn add_attr(&self, attr: Attribute) {
        unsafe {
            LLVMAddAttribute(self.to_ref(), attr.bits() as c_uint)
        }
    }

    fn get_attr(&self) -> Attribute {
        unsafe {
            Attribute::from_bits_truncate(LLVMGetAttribute(self.to_ref()) as u32)
        }
    }

    fn remove_attr(&self, attr: Attribute) {
        unsafe {
            LLVMRemoveAttribute(self.to_ref(), attr.bits() as c_uint)
        }
    }

    fn set_alignment(&self, align: u32) {
        unsafe {
            LLVMSetParamAlignment(self.to_ref(), align)
        }
    }
}

new_ref_type!(ArgumentRef for ValueRef
              implementing
              Value,
              Argument
              );

pub struct ArgsIter {
    current: Option<ValueRef>,
    func: ValueRef,
    pos: u32,
    size: u32
}

impl Iterator for ArgsIter {
    type Item = ArgumentRef;

    fn next(&mut self) -> Option<ArgumentRef> {
        let cur = match self.current {
            Some(cur) => {
                cur
            },
            None => {
                unsafe {
                    self.size = LLVMCountParams(self.func);
                    self.pos = 0;
                    if self.size == 0 {
                        return None
                    }
                    LLVMGetFirstParam(self.func)
                }
            }
        };

        if self.pos >= self.size {
            return None
        }

        unsafe {
            if self.pos < self.size - 1 {
                self.current = Some(LLVMGetNextParam(cur))
            }

            self.pos += 1;

            Some(ArgumentRef::from_ref(cur))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (pos, size) = match self.current {
            Some(_) => (self.pos, self.size),
            None => unsafe { (0, LLVMCountParams(self.func)) }
        };

        ((size - pos) as usize, Some((size - pos) as usize))
    }
}

pub mod ffi {
    use ::Bool;
    use core::*;
    use libc::{c_char, c_int, c_longlong, c_uint, c_ulonglong, uint64_t};

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


        /* Operations on scalar constants */

        /**
         * Obtain a constant value for an integer type.
         *
         * The returned value corresponds to a llvm::ConstantInt.
         */
        pub fn LLVMConstInt(IntTy: TypeRef, N: c_ulonglong, SignExtend: Bool)
                            -> ValueRef;

        /**
         * Obtain a constant value for an integer of arbitrary precision.
         */
        pub fn LLVMConstIntOfArbitraryPrecision(IntTy: TypeRef,
                                                NumWords: c_uint,
                                                Words: *const uint64_t)
                                                -> ValueRef;

        /**
         * Obtain a constant value for an integer parsed from a string.
         *
         * A similar API, LLVMConstIntOfStringAndSize is also available. If the
         * string's length is available, it is preferred to call that function
         * instead.
         */
        pub fn LLVMConstIntOfString(IntTy: TypeRef, Text: *const c_char, Radix: u8)
                                    -> ValueRef;

        /**
         * Obtain a constant value for an integer parsed from a string with
         * specified length.
         */
        pub fn LLVMConstIntOfStringAndSize(IntTy: TypeRef,
                                           Text: *const c_char,
                                           SLen: c_uint,
                                           Radix: u8)
                                           -> ValueRef;

        /**
         * Obtain a constant value referring to a double floating point value.
         */
        pub fn LLVMConstReal(RealTy: TypeRef, N: f64) -> ValueRef;

        /**
         * Obtain a constant for a floating point value parsed from a string.
         *
         * A similar API, LLVMConstRealOfStringAndSize is also available. It
         * should be used if the input string's length is known.
         */
        pub fn LLVMConstRealOfString(RealTy: TypeRef, Text: *const c_char)
                                     -> ValueRef;

        /**
         * Obtain a constant for a floating point value parsed from a string.
         */
        pub fn LLVMConstRealOfStringAndSize(RealTy: TypeRef,
                                            Text: *const c_char,
                                            SLen: c_uint)
                                            -> ValueRef;

        /**
         * Obtain the zero extended value for an integer constant value.
         */
        pub fn LLVMConstIntGetZExtValue(ConstantVal: ValueRef) -> c_ulonglong;

        /**
         * Obtain the sign extended value for an integer constant value.
         */
        pub fn LLVMConstIntGetSExtValue(ConstantVal: ValueRef) -> c_longlong;

        /**
         * Obtain the double value for an floating point constant value.
         * losesInfo indicates if some precision was lost in the conversion.
         */
        pub fn LLVMConstRealGetDouble(ConstantVal: ValueRef, losesInfo: *mut Bool) -> f64;


        /* Operations on functions */

        /**
         * Remove a function from its containing module and deletes it.
         */
        pub fn LLVMDeleteFunction(Fn: ValueRef);

        /**
         * Obtain the ID number from a function instance.
         */
        pub fn LLVMGetIntrinsicID(Fn: ValueRef) -> c_uint;

        /**
         * Obtain the calling function of a function.
         *
         * The returned value corresponds to the LLVMCallConv enumeration.
         */
        pub fn LLVMGetFunctionCallConv(Fn: ValueRef) -> c_uint;

        /**
         * Set the calling convention of a function.
         */
        pub fn LLVMSetFunctionCallConv(Fn: ValueRef, CC: c_uint);

        /**
         * Obtain the name of the garbage collector to use during code
         * generation.
         */
        pub fn LLVMGetGC(Fn: ValueRef) -> *const c_char;

        /**
         * Define the garbage collector to use during code generation.
         */
        pub fn LLVMSetGC(Fn: ValueRef, Name: *const c_char);

        /**
         * Add an attribute to a function.
         */
        pub fn LLVMAddFunctionAttr(Fn: ValueRef, PA: c_ulonglong);

        /**
         * Add a target-dependent attribute to a fuction
         */
        pub fn LLVMAddTargetDependentFunctionAttr(Fn: ValueRef,
                                                  A: *const c_char,
                                                  V: *const c_char);

        /**
         * Obtain an attribute from a function.
         */
        pub fn LLVMGetFunctionAttr(Fn: ValueRef) -> c_ulonglong;

        /**
         * Remove an attribute from a function.
         */
        pub fn LLVMRemoveFunctionAttr(Fn: ValueRef, PA: c_ulonglong);


        /* Operations on parameters */

        /**
         * Obtain the number of parameters in a function.
         */
        pub fn LLVMCountParams(Fn: ValueRef) -> c_uint;

        /**
         * Obtain the parameters in a function.
         *
         * The takes a pointer to a pre-allocated array of LLVMValueRef that is
         * at least LLVMCountParams() long. This array will be filled with
         * LLVMValueRef instances which correspond to the parameters the
         * function receives. Each LLVMValueRef corresponds to a llvm::Argument
         * instance.
         */
        pub fn LLVMGetParams(Fn: ValueRef, Params: *const ValueRef);

        /**
         * Obtain the parameter at the specified index.
         *
         * Parameters are indexed from 0.
         */
        pub fn LLVMGetParam(Fn: ValueRef, Index: c_uint) -> ValueRef;

        /**
         * Obtain the function to which this argument belongs.
         *
         * Unlike other functions in this group, this one takes an LLVMValueRef
         * that corresponds to a llvm::Attribute.
         *
         * The returned LLVMValueRef is the llvm::Function to which this
         * argument belongs.
         */
        pub fn LLVMGetParamParent(Inst: ValueRef) -> ValueRef;

        /**
         * Obtain the first parameter to a function.
         */
        pub fn LLVMGetFirstParam(Fn: ValueRef) -> ValueRef;

        /**
         * Obtain the last parameter to a function.
         */
        pub fn LLVMGetLastParam(Fn: ValueRef) -> ValueRef;

        /**
         * Obtain the next parameter to a function.
         *
         * This takes an LLVMValueRef obtained from LLVMGetFirstParam() (which is
              * actually a wrapped iterator) and obtains the next parameter from the
         * underlying iterator.
         */
        pub fn LLVMGetNextParam(Arg: ValueRef) -> ValueRef;

        /**
         * Obtain the previous parameter to a function.
         *
         * This is the opposite of LLVMGetNextParam().
         */
        pub fn LLVMGetPreviousParam(Arg: ValueRef) -> ValueRef;

        /**
         * Add an attribute to a function argument.
         */
        pub fn LLVMAddAttribute(Arg: ValueRef, PA: c_uint);

        /**
         * Remove an attribute from a function argument.
         */
        pub fn LLVMRemoveAttribute(Arg: ValueRef, PA: c_uint);

        /**
         * Get an attribute from a function argument.
         */
        pub fn LLVMGetAttribute(Arg: ValueRef) -> c_uint;

        /**
         * Set the alignment for a function parameter.
         */
        pub fn LLVMSetParamAlignment(Arg: ValueRef, align: c_uint);
    }
}
