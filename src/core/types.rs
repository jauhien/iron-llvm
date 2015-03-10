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

pub trait IntType : Type{
    fn int1_type_in_context(ctx: context::Context) -> Self;
    fn int8_type_in_context(ctx: context::Context) -> Self;
    fn int16_type_in_context(ctx: context::Context) -> Self;
    fn int32_type_in_context(ctx: context::Context) -> Self;
    fn int64_type_in_context(ctx: context::Context) -> Self;
    fn int_type_in_context(ctx: context::Context, num_bits: u32) -> Self;

    fn int1_type() -> Self;
    fn int8_type() -> Self;
    fn int16_type() -> Self;
    fn int32_type() -> Self;
    fn int64_type() -> Self;
    fn int_type(num_bits: u32) -> Self;

    fn get_width(&self) -> u32;
}

pub enum IntTypeRef {
    Ref(TypeRef)
}

impl Type for IntTypeRef {
    fn get_ref(&self) -> TypeRef {
        match *self {
            IntTypeRef::Ref(rf) => rf
        }
    }
}

impl IntType for IntTypeRef {
    fn int1_type_in_context(ctx: context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt1TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int8_type_in_context(ctx: context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt8TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int16_type_in_context(ctx: context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt16TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int32_type_in_context(ctx: context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt32TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int64_type_in_context(ctx: context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt64TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int_type_in_context(ctx: context::Context, num_bits: u32) -> IntTypeRef {
        let rf = unsafe {
            LLVMIntTypeInContext(ctx.get_ref(), num_bits)
        };

        IntTypeRef::Ref(rf)
    }

    fn int1_type() -> IntTypeRef {
        let rf = unsafe {
            LLVMInt1Type()
        };

        IntTypeRef::Ref(rf)
    }

    fn int8_type() -> IntTypeRef {
        let rf = unsafe {
            LLVMInt8Type()
        };

        IntTypeRef::Ref(rf)
    }

    fn int16_type() -> IntTypeRef {
        let rf = unsafe {
            LLVMInt16Type()
        };

        IntTypeRef::Ref(rf)
    }

    fn int32_type() -> IntTypeRef {
        let rf = unsafe {
            LLVMInt32Type()
        };

        IntTypeRef::Ref(rf)
    }

    fn int64_type() -> IntTypeRef {
        let rf = unsafe {
            LLVMInt64Type()
        };

        IntTypeRef::Ref(rf)
    }

    fn int_type(num_bits: u32) -> IntTypeRef {
        let rf = unsafe {
            LLVMIntType(num_bits)
        };

        IntTypeRef::Ref(rf)
    }

    fn get_width(&self) -> u32 {
        unsafe {
            LLVMGetIntTypeWidth(self.get_ref())
        }
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

        /**
         * Obtain an integer type from the global context with a specified bit
         * width.
         */
        pub fn LLVMInt1Type() -> TypeRef;
        pub fn LLVMInt8Type() -> TypeRef;
        pub fn LLVMInt16Type() -> TypeRef;
        pub fn LLVMInt32Type() -> TypeRef;
        pub fn LLVMInt64Type() -> TypeRef;
        pub fn LLVMIntType(NumBits: c_uint) -> TypeRef;

        pub fn LLVMGetIntTypeWidth(IntegerTy: TypeRef) -> c_uint;
    }
}
