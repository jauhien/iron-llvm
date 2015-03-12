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

use libc::{c_char, c_uint};

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
    fn int1_type_in_context(ctx: &context::Context) -> Self;
    fn int8_type_in_context(ctx: &context::Context) -> Self;
    fn int16_type_in_context(ctx: &context::Context) -> Self;
    fn int32_type_in_context(ctx: &context::Context) -> Self;
    fn int64_type_in_context(ctx: &context::Context) -> Self;
    fn int_type_in_context(ctx: &context::Context, num_bits: u32) -> Self;

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
    fn int1_type_in_context(ctx: &context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt1TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int8_type_in_context(ctx: &context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt8TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int16_type_in_context(ctx: &context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt16TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int32_type_in_context(ctx: &context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt32TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int64_type_in_context(ctx: &context::Context) -> IntTypeRef {
        let rf = unsafe {
            LLVMInt64TypeInContext(ctx.get_ref())
        };

        IntTypeRef::Ref(rf)
    }

    fn int_type_in_context(ctx: &context::Context, num_bits: u32) -> IntTypeRef {
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

pub trait FloatType : Type {
    fn half_type_in_context(ctx: &context::Context) -> Self;
    fn float_type_in_context(ctx: &context::Context) -> Self;
    fn double_type_in_context(ctx: &context::Context) -> Self;
    fn x86fp80_type_in_context(ctx: &context::Context) -> Self;
    fn fp128_type_in_context(ctx: &context::Context) -> Self;
    fn ppcfp128_type_in_context(ctx: &context::Context) -> Self;

    fn half_type() -> Self;
    fn float_type() -> Self;
    fn double_type() -> Self;
    fn x86fp80_type() -> Self;
    fn fp128_type() -> Self;
    fn ppcfp128_type() -> Self;
}

pub enum FloatTypeRef {
    Ref(TypeRef)
}

impl Type for FloatTypeRef {
    fn get_ref(&self) -> TypeRef {
        match *self {
            FloatTypeRef::Ref(rf) => rf
        }
    }
}

impl FloatType for FloatTypeRef {
    fn half_type_in_context(ctx: &context::Context) -> Self {
        let rf = unsafe {
            LLVMHalfTypeInContext(ctx.get_ref())
        };

        FloatTypeRef::Ref(rf)
    }

    fn float_type_in_context(ctx: &context::Context) -> Self {
        let rf = unsafe {
            LLVMFloatTypeInContext(ctx.get_ref())
        };

        FloatTypeRef::Ref(rf)
    }

    fn double_type_in_context(ctx: &context::Context) -> Self {
        let rf = unsafe {
            LLVMDoubleTypeInContext(ctx.get_ref())
        };

        FloatTypeRef::Ref(rf)
    }

    fn x86fp80_type_in_context(ctx: &context::Context) -> Self {
        let rf = unsafe {
            LLVMX86FP80TypeInContext(ctx.get_ref())
        };

        FloatTypeRef::Ref(rf)
    }

    fn fp128_type_in_context(ctx: &context::Context) -> Self {
        let rf = unsafe {
            LLVMFP128TypeInContext(ctx.get_ref())
        };

        FloatTypeRef::Ref(rf)
    }

    fn ppcfp128_type_in_context(ctx: &context::Context) -> Self{
        let rf = unsafe {
            LLVMPPCFP128TypeInContext(ctx.get_ref())
        };

        FloatTypeRef::Ref(rf)
    }

    fn half_type() -> Self {
        let rf = unsafe {
            LLVMHalfType()
        };

        FloatTypeRef::Ref(rf)
    }

    fn float_type() -> Self {
        let rf = unsafe {
            LLVMFloatType()
        };

        FloatTypeRef::Ref(rf)
    }

    fn double_type() -> Self {
        let rf = unsafe {
            LLVMDoubleType()
        };

        FloatTypeRef::Ref(rf)
    }

    fn x86fp80_type() -> Self {
        let rf = unsafe {
            LLVMX86FP80Type()
        };

        FloatTypeRef::Ref(rf)
    }

    fn fp128_type() -> Self {
        let rf = unsafe {
            LLVMFP128Type()
        };

        FloatTypeRef::Ref(rf)
    }

    fn ppcfp128_type() -> Self{
        let rf = unsafe {
            LLVMPPCFP128Type()
        };

        FloatTypeRef::Ref(rf)
    }
}

pub trait FunctionType : Type {
    fn function_type(return_type: &Type, param_types: &[TypeRef], is_var_arg: bool) -> Self;
    fn is_var_arg(&self) -> bool;
    fn get_return_type(&self) -> TypeRef;
    fn count_param_types(&self) -> u32;
    fn get_param_types(&self) -> Vec<TypeRef>;
}

pub enum FunctionTypeRef {
    Ref(TypeRef)
}

impl Type for FunctionTypeRef {
    fn get_ref(&self) -> TypeRef {
        match *self {
            FunctionTypeRef::Ref(rf) => rf
        }
    }
}

impl FunctionType for FunctionTypeRef {
    fn function_type(return_type: &Type, param_types: &[TypeRef], is_var_arg: bool) -> FunctionTypeRef {
        let rf = unsafe {
            LLVMFunctionType(return_type.get_ref(),
                             param_types.as_ptr(),
                             param_types.len() as c_uint,
                             is_var_arg as ::Bool)
        };

        FunctionTypeRef::Ref(rf)
    }

    fn is_var_arg(&self) -> bool {
        unsafe {
            LLVMIsFunctionVarArg(self.get_ref()) > 0
        }
    }

    fn get_return_type(&self) -> TypeRef {
        unsafe {
            LLVMGetReturnType(self.get_ref())
        }
    }

    fn count_param_types(&self) -> u32 {
        unsafe {
            LLVMCountParamTypes(self.get_ref())
        }
    }

    fn get_param_types(&self) -> Vec<TypeRef> {
        let params_count = self.count_param_types();
        let mut buf : Vec<TypeRef> = Vec::with_capacity(params_count as usize);
        let p = buf.as_mut_ptr();
        unsafe {
            std::mem::forget(buf);
            LLVMGetParamTypes(self.get_ref(), p);
            Vec::from_raw_parts(p, params_count as usize, params_count as usize)
        }
    }
}

pub trait StructType : Type {
    fn struct_type_in_context(ctx: &context::Context, element_types: &[TypeRef], packed: bool) -> Self;
    fn struct_type(element_types: &[TypeRef], packed: bool) -> Self;
    fn struct_create_named(ctx: &context::Context, name: &str) -> Self;
    fn get_name(&self) -> String;
    fn set_body(&self, element_types: &[TypeRef], packed: bool);
    fn count_element_types(&self) -> u32;
    fn get_element_types(&self) -> Vec<TypeRef>;
    fn is_packed(&self) -> bool;
    fn is_opaque(&self) -> bool;
}

pub enum StructTypeRef {
    Ref(TypeRef)
}

impl Type for StructTypeRef {
    fn get_ref(&self) -> TypeRef {
        match *self {
            StructTypeRef::Ref(rf) => rf
        }
    }
}

impl StructType for StructTypeRef {
    fn struct_type_in_context(ctx: &context::Context, element_types: &[TypeRef], packed: bool) -> StructTypeRef {
        let rf = unsafe {
            LLVMStructTypeInContext(ctx.get_ref(),
                                    element_types.as_ptr(),
                                    element_types.len() as c_uint,
                                    packed as ::Bool)
        };

        StructTypeRef::Ref(rf)
    }

    fn struct_type(element_types: &[TypeRef], packed: bool) -> StructTypeRef {
        let rf = unsafe {
            LLVMStructType(element_types.as_ptr(),
                           element_types.len() as c_uint,
                           packed as ::Bool)
        };

        StructTypeRef::Ref(rf)
    }

    fn struct_create_named(ctx: &context::Context, name: &str) -> StructTypeRef {
        let rf = unsafe {
            LLVMStructCreateNamed(ctx.get_ref(),
                                  name.as_ptr() as *const c_char)
        };

        StructTypeRef::Ref(rf)
    }

    fn get_name(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMGetStructName(self.get_ref()))
        };
        String::from_utf8_lossy(buf.to_bytes()).into_owned()
    }

    fn set_body(&self, element_types: &[TypeRef], packed: bool) {
        unsafe {
            LLVMStructSetBody(self.get_ref(),
                              element_types.as_ptr(),
                              element_types.len() as c_uint,
                              packed as ::Bool)
        }
    }

    fn count_element_types(&self) -> u32 {
        unsafe {
            LLVMCountStructElementTypes(self.get_ref())
        }
    }

    fn get_element_types(&self) -> Vec<TypeRef> {
        let element_count = self.count_element_types();
        let mut buf : Vec<TypeRef> = Vec::with_capacity(element_count as usize);
        let p = buf.as_mut_ptr();
        unsafe {
            std::mem::forget(buf);
            LLVMGetStructElementTypes(self.get_ref(), p);
            Vec::from_raw_parts(p, element_count as usize, element_count as usize)
        }
    }

    fn is_packed(&self) -> bool {
        unsafe {
            LLVMIsPackedStruct(self.get_ref()) > 0
        }
    }

    fn is_opaque(&self) -> bool {
        unsafe {
            LLVMIsOpaqueStruct(self.get_ref()) > 0
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
         * Things that don't have a size are abstract types, labels, and void.
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

        /**
         * Obtain a 16-bit floating point type from a context.
         */
        pub fn LLVMHalfTypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Obtain a 32-bit floating point type from a context.
         */
        pub fn LLVMFloatTypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Obtain a 64-bit floating point type from a context.
         */
        pub fn LLVMDoubleTypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Obtain a 80-bit floating point type (X87) from a context.
         */
        pub fn LLVMX86FP80TypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Obtain a 128-bit floating point type (112-bit mantissa) from a
         * context.
         */
        pub fn LLVMFP128TypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Obtain a 128-bit floating point type (two 64-bits) from a context.
         */
        pub fn LLVMPPCFP128TypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Obtain a floating point type from the global context.
         *
         * These map to the functions in this group of the same name.
         */
        pub fn LLVMHalfType() -> TypeRef;
        pub fn LLVMFloatType() -> TypeRef;
        pub fn LLVMDoubleType() -> TypeRef;
        pub fn LLVMX86FP80Type() -> TypeRef;
        pub fn LLVMFP128Type() -> TypeRef;
        pub fn LLVMPPCFP128Type() -> TypeRef;

        /* Operations on function types */

        /**
         * Obtain a function type consisting of a specified signature.
         *
         * The function is defined as a tuple of a return Type, a list of
         * parameter types, and whether the function is variadic.
         */
        pub fn LLVMFunctionType(ReturnType: TypeRef,
                                ParamTypes: *const TypeRef,
                                ParamCount: c_uint,
                                IsVarArg: Bool)
                                -> TypeRef;

        /**
         * Returns whether a function type is variadic.
         */
        pub fn LLVMIsFunctionVarArg(FunctionTy: TypeRef) -> Bool;

        /**
         * Obtain the Type this function Type returns.
         */
        pub fn LLVMGetReturnType(FunctionTy: TypeRef) -> TypeRef;

        /**
         * Obtain the number of parameters this function accepts.
         */
        pub fn LLVMCountParamTypes(FunctionTy: TypeRef) -> c_uint;

        /**
         * Obtain the types of a function's parameters.
         *
         * The Dest parameter should point to a pre-allocated array of
         * LLVMTypeRef at least LLVMCountParamTypes() large. On return, the
         * first LLVMCountParamTypes() entries in the array will be populated
         * with LLVMTypeRef instances.
         *
         * @param FunctionTy The function type to operate on.
         * @param Dest Memory address of an array to be filled with result.
         */
        pub fn LLVMGetParamTypes(FunctionTy: TypeRef, Dest: *mut TypeRef);


        /* Operations on struct types */

        /**
         * Create a new structure type in a context.
         *
         * A structure is specified by a list of inner elements/types and
         * whether these can be packed together.
         */
        pub fn LLVMStructTypeInContext(C: ContextRef,
                                       ElementTypes: *const TypeRef,
                                       ElementCount: c_uint,
                                       Packed: Bool)
                                       -> TypeRef;

        /**
         * Create a new structure type in the global context.
         */
        pub fn LLVMStructType(ElementTypes: *const TypeRef,
                              ElementCount: c_uint,
                              Packed: Bool)
                              -> TypeRef;

        /**
         * Create an empty structure in a context having a specified name.
         */
        pub fn LLVMStructCreateNamed(C: ContextRef,
                                     Name: *const c_char)
                                     -> TypeRef;

        /**
         * Obtain the name of a structure.
         */
        pub fn LLVMGetStructName(StructTy: TypeRef) -> *const c_char;

        /**
         * Set the contents of a structure type.
         */
        pub fn LLVMStructSetBody(StructTy: TypeRef,
                                 ElementTypes: *const TypeRef,
                                 ElementCount: c_uint,
                                 Packed: Bool);

        /**
         * Get the number of elements defined inside the structure.
         */
        pub fn LLVMCountStructElementTypes(StructTy: TypeRef) -> c_uint;

        /**
         * Get the elements within a structure.
         *
         * The function is passed the address of a pre-allocated array of
         * LLVMTypeRef at least LLVMCountStructElementTypes() long. After
         * invocation, this array will be populated with the structure's
         * elements. The objects in the destination array will have a lifetime
         * of the structure type itself, which is the lifetime of the context it
         * is contained in.
         */
        pub fn LLVMGetStructElementTypes(StructTy: TypeRef,
                                         Dest: *mut TypeRef);

        /**
         * Determine whether a structure is packed.
         */
        pub fn LLVMIsPackedStruct(StructTy: TypeRef) -> Bool;

        /**
         * Determine whether a structure is opaque.
         */
        pub fn LLVMIsOpaqueStruct(StructTy: TypeRef) -> Bool;
    }
}
