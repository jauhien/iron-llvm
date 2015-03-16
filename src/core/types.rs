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

use ::{LLVMRef, LLVMRefCtor};
use core;
use core::{context, TypeKind, TypeRef};
use self::ffi::*;

pub trait Type : LLVMRef<TypeRef> {
    fn get_kind(&self) -> TypeKind {
        unsafe {
            LLVMGetTypeKind(self.to_ref())
        }
    }

    fn is_sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(self.to_ref()) > 0
        }
    }

    fn get_context(&self) -> context::Context {
        unsafe {
            let ctx = LLVMGetTypeContext(self.to_ref());
            context::Context::from_ref(ctx)
        }
    }

    fn dump(&self) {
        unsafe {
            LLVMDumpType(self.to_ref());
        }
    }

    fn print_to_string(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMPrintTypeToString(self.to_ref()))
        };
        let result = String::from_utf8_lossy(buf.to_bytes()).into_owned();
        unsafe { core::ffi::LLVMDisposeMessage(buf.as_ptr()); }
        result
    }
}

pub trait TypeCtor : LLVMRefCtor<TypeRef> {}

impl LLVMRef<TypeRef> for TypeRef {
    fn to_ref(&self) -> TypeRef {
        *self
    }
}

impl LLVMRefCtor<TypeRef> for TypeRef {
    unsafe fn from_ref(rf: TypeRef) -> TypeRef {
        rf
    }
}

impl Type for TypeRef {}
impl TypeCtor for TypeRef {}

pub trait IntTypeCtor : TypeCtor {
    fn get_int1_in_context(ctx: &context::Context) -> Self  {
        unsafe {
            Self::from_ref(LLVMInt1TypeInContext(ctx.to_ref()))
        }
    }

    fn get_int8_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMInt8TypeInContext(ctx.to_ref()))
        }
    }

    fn get_int16_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMInt16TypeInContext(ctx.to_ref()))
        }
    }

    fn get_int32_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMInt32TypeInContext(ctx.to_ref()))
        }
    }

    fn get_int64_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMInt64TypeInContext(ctx.to_ref()))
        }
    }

    fn get_int_in_context(ctx: &context::Context, num_bits: u32) -> Self {
        unsafe {
            Self::from_ref(LLVMIntTypeInContext(ctx.to_ref(), num_bits))
        }
    }

    fn get_int1() -> Self {
        unsafe {
            Self::from_ref(LLVMInt1Type())
        }
    }

    fn get_int8() -> Self {
        unsafe {
            Self::from_ref(LLVMInt8Type())
        }
    }

    fn get_int16() -> Self {
        unsafe {
            Self::from_ref(LLVMInt16Type())
        }
    }

    fn get_int32() -> Self {
        unsafe {
            Self::from_ref(LLVMInt32Type())
        }
    }

    fn get_int64() -> Self {
        unsafe {
            Self::from_ref(LLVMInt64Type())
        }
    }

    fn get_int(num_bits: u32) -> Self {
        unsafe {
            Self::from_ref(LLVMIntType(num_bits))
        }
    }

}

pub trait IntType : Type {
    fn get_width(&self) -> u32 {
        unsafe {
            LLVMGetIntTypeWidth(self.to_ref())
        }
    }
}

new_ref_type!(IntTypeRef for TypeRef implementing Type, IntType, TypeCtor, IntTypeCtor);

pub trait RealTypeCtor : TypeCtor {
    fn get_half_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMHalfTypeInContext(ctx.to_ref()))
        }
    }

    fn get_float_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMFloatTypeInContext(ctx.to_ref()))
        }
    }

    fn get_double_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMDoubleTypeInContext(ctx.to_ref()))
        }
    }

    fn get_x86fp80_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMX86FP80TypeInContext(ctx.to_ref()))
        }
    }

    fn get_fp128_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMFP128TypeInContext(ctx.to_ref()))
        }
    }

    fn get_ppcfp128_in_context(ctx: &context::Context) -> Self{
        unsafe {
            Self::from_ref(LLVMPPCFP128TypeInContext(ctx.to_ref()))
        }
    }

    fn get_half() -> Self {
        unsafe {
            Self::from_ref(LLVMHalfType())
        }
    }

    fn get_float() -> Self {
        unsafe {
            Self::from_ref(LLVMFloatType())
        }
    }

    fn get_double() -> Self {
        unsafe {
            Self::from_ref(LLVMDoubleType())
        }
    }

    fn get_x86fp80() -> Self {
        unsafe {
            Self::from_ref(LLVMX86FP80Type())
        }
    }

    fn get_fp128() -> Self {
        unsafe {
            Self::from_ref(LLVMFP128Type())
        }
    }

    fn get_ppcfp128() -> Self{
        unsafe {
            Self::from_ref(LLVMPPCFP128Type())
        }
    }
}

pub trait RealType : Type {}

new_ref_type!(RealTypeRef for TypeRef implementing Type, RealType, TypeCtor, RealTypeCtor);

pub trait FunctionTypeCtor : TypeCtor {
    fn get(return_type: &Type, param_types: &[TypeRef], is_var_arg: bool) -> Self {
        unsafe {
            Self::from_ref(LLVMFunctionType(return_type.to_ref(),
                                            param_types.as_ptr(),
                                            param_types.len() as c_uint,
                                            is_var_arg as ::Bool))
        }
    }
}

pub trait FunctionType : Type {
    fn is_var_arg(&self) -> bool {
        unsafe {
            LLVMIsFunctionVarArg(self.to_ref()) > 0
        }
    }

    fn get_return_type(&self) -> TypeRef {
        unsafe {
            LLVMGetReturnType(self.to_ref())
        }
    }

    fn count_param_types(&self) -> u32 {
        unsafe {
            LLVMCountParamTypes(self.to_ref())
        }
    }

    fn get_param_types(&self) -> Vec<TypeRef> {
        let params_count = self.count_param_types();
        let mut buf : Vec<TypeRef> = Vec::with_capacity(params_count as usize);
        let p = buf.as_mut_ptr();
        unsafe {
            std::mem::forget(buf);
            LLVMGetParamTypes(self.to_ref(), p);
            Vec::from_raw_parts(p, params_count as usize, params_count as usize)
        }
    }
}

new_ref_type!(FunctionTypeRef for TypeRef implementing Type, FunctionType, TypeCtor, FunctionTypeCtor);

pub trait StructTypeCtor : TypeCtor {
    fn get_in_context(ctx: &context::Context, element_types: &[TypeRef], packed: bool) -> Self {
        unsafe {
            Self::from_ref(LLVMStructTypeInContext(ctx.to_ref(),
                                                   element_types.as_ptr(),
                                                   element_types.len() as c_uint,
                                                   packed as ::Bool))
        }
    }

    fn get(element_types: &[TypeRef], packed: bool) -> Self {
        unsafe {
            Self::from_ref(LLVMStructType(element_types.as_ptr(),
                                          element_types.len() as c_uint,
                                          packed as ::Bool))
        }
    }

    fn new_named(ctx: &context::Context, name: &str) -> Self {
        unsafe {
            Self::from_ref(LLVMStructCreateNamed(ctx.to_ref(),
                                                 name.as_ptr() as *const c_char))
        }
    }
}

pub trait StructType : Type {
    fn get_name(&self) -> String {
        let buf = unsafe {
            std::ffi::CStr::from_ptr(LLVMGetStructName(self.to_ref()))
        };
        String::from_utf8_lossy(buf.to_bytes()).into_owned()
    }

    fn set_body(&self, element_types: &[TypeRef], packed: bool) {
        unsafe {
            LLVMStructSetBody(self.to_ref(),
                              element_types.as_ptr(),
                              element_types.len() as c_uint,
                              packed as ::Bool)
        }
    }

    fn count_element_types(&self) -> u32 {
        unsafe {
            LLVMCountStructElementTypes(self.to_ref())
        }
    }

    fn get_element_types(&self) -> Vec<TypeRef> {
        let element_count = self.count_element_types();
        let mut buf : Vec<TypeRef> = Vec::with_capacity(element_count as usize);
        let p = buf.as_mut_ptr();
        unsafe {
            std::mem::forget(buf);
            LLVMGetStructElementTypes(self.to_ref(), p);
            Vec::from_raw_parts(p, element_count as usize, element_count as usize)
        }
    }

    fn is_packed(&self) -> bool {
        unsafe {
            LLVMIsPackedStruct(self.to_ref()) > 0
        }
    }

    fn is_opaque(&self) -> bool {
        unsafe {
            LLVMIsOpaqueStruct(self.to_ref()) > 0
        }
    }
}

new_ref_type!(StructTypeRef for TypeRef implementing Type, StructType, TypeCtor, StructTypeCtor);

pub trait SequentialTypeCtor : TypeCtor {}

pub trait SequentialType : Type {
    fn get_element_type(&self) -> TypeRef {
        unsafe {
            LLVMGetElementType(self.to_ref())
        }
    }
}

pub trait ArrayTypeCtor : SequentialTypeCtor {
    fn get(element_type: &Type, element_count: u32) -> Self {
        unsafe {
            Self::from_ref(LLVMArrayType(element_type.to_ref(),
                                         element_count))
        }
    }
}

pub trait ArrayType : SequentialType {
    fn get_length(&self) -> u32 {
        unsafe {
            LLVMGetArrayLength(self.to_ref())
        }
    }
}

pub trait PointerTypeCtor : SequentialTypeCtor {
    fn get(element_type: &Type, address_space: u32) -> Self {
        unsafe {
            Self::from_ref(LLVMPointerType(element_type.to_ref(),
                                           address_space))
        }
    }
}

pub trait PointerType : SequentialType {
    fn get_address_space(&self) -> u32 {
        unsafe {
            LLVMGetPointerAddressSpace(self.to_ref())
        }
    }
}

pub trait VectorTypeCtor : SequentialTypeCtor {
    fn get(element_type: &Type, element_count: u32) -> Self {
        unsafe {
            Self::from_ref(LLVMVectorType(element_type.to_ref(),
                                           element_count))
        }
    }
}

pub trait VectorType : SequentialType {
    fn get_size(&self) -> u32 {
        unsafe {
            LLVMGetVectorSize(self.to_ref())
        }
    }
}

new_ref_type!(ArrayTypeRef for TypeRef
              implementing
              Type,
              SequentialType,
              ArrayType,
              TypeCtor,
              SequentialTypeCtor,
              ArrayTypeCtor);

new_ref_type!(PointerTypeRef for TypeRef
              implementing
              Type,
              SequentialType,
              PointerType,
              TypeCtor,
              SequentialTypeCtor,
              PointerTypeCtor);

new_ref_type!(VectorTypeRef for TypeRef
              implementing
              Type,
              SequentialType,
              VectorType,
              TypeCtor,
              SequentialTypeCtor,
              VectorTypeCtor);

pub trait VoidTypeCtor : TypeCtor {
    fn get_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMVoidTypeInContext(ctx.to_ref()))
        }
    }

    fn get() -> Self {
        unsafe {
            Self::from_ref(LLVMVoidType())
        }
    }
}

pub trait VoidType : std::marker::MarkerTrait {}

pub trait LabelTypeCtor : TypeCtor {
    fn get_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMLabelTypeInContext(ctx.to_ref()))
        }
    }

    fn get() -> Self {
        unsafe {
            Self::from_ref(LLVMLabelType())
        }
    }
}

pub trait LabelType : std::marker::MarkerTrait {}

pub trait X86MMXTypeCtor : TypeCtor {
    fn get_in_context(ctx: &context::Context) -> Self {
        unsafe {
            Self::from_ref(LLVMX86MMXTypeInContext(ctx.to_ref()))
        }
    }

    fn get() -> Self {
        unsafe {
            Self::from_ref(LLVMX86MMXType())
        }
    }
}

pub trait X86MMXType : std::marker::MarkerTrait {}

new_ref_type!(VoidTypeRef for TypeRef implementing Type, VoidType, TypeCtor, VoidTypeCtor);
new_ref_type!(LabelTypeRef for TypeRef implementing Type, LabelType, TypeCtor, LabelTypeCtor);
new_ref_type!(X86MMXTypeRef for TypeRef implementing Type, X86MMXType, TypeCtor, X86MMXTypeCtor);

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


        /* Operations on array, pointer, and vector types (sequence types) */


        /**
         * Obtain the type of elements within a sequential type.
         *
         * This works on array, vector, and pointer types.
         */
        pub fn LLVMGetElementType(Ty: TypeRef) -> TypeRef;

        /**
         * Create a fixed size array type that refers to a specific type.
         *
         * The created type will exist in the context that its element type
         * exists in.
         */
        pub fn LLVMArrayType(ElementType: TypeRef, ElementCount: c_uint)
                             -> TypeRef;

        /**
         * Obtain the length of an array type.
         *
         * This only works on types that represent arrays.
         */
        pub fn LLVMGetArrayLength(ArrayTy: TypeRef) -> c_uint;

        /**
         * Create a pointer type that points to a defined type.
         *
         * The created type will exist in the context that its pointee type
         * exists in.
         */
        pub fn LLVMPointerType(ElementType: TypeRef, AddressSpace: c_uint)
                               -> TypeRef;

        /**
         * Obtain the address space of a pointer type.
         *
         * This only works on types that represent pointers.
         */
        pub fn LLVMGetPointerAddressSpace(PointerTy: TypeRef) -> c_uint;

        /**
         * Create a vector type that contains a defined type and has a specific
         * number of elements.
         *
         * The created type will exist in the context thats its element type
         * exists in.
         */
        pub fn LLVMVectorType(ElementType: TypeRef, ElementCount: c_uint)
                              -> TypeRef;

        /**
         * Obtain the number of elements in a vector type.
         *
         * This only works on types that represent vectors.
         */
        pub fn LLVMGetVectorSize(VectorTy: TypeRef) -> c_uint;


        /* Operations on other types */

        /**
         * Create a void type in a context.
         */
        pub fn LLVMVoidTypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Create a label type in a context.
         */
        pub fn LLVMLabelTypeInContext(C: ContextRef) -> TypeRef;

        /**
         * Create a X86 MMX type in a context.
         */
        pub fn LLVMX86MMXTypeInContext(C: ContextRef) -> TypeRef;

        /**
         * These are similar to the above functions except they operate on the
         * global context.
         */
        pub fn LLVMVoidType() -> TypeRef;
        pub fn LLVMLabelType() -> TypeRef;
        pub fn LLVMX86MMXType() -> TypeRef;
    }
}
