// Copyright 2015 Jauhien Piatlicki.
//
// Copyright 2012-2015 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Core LLVM
// LLVM-C header Core.h

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use libc::c_void;

pub mod context;
pub mod types;
pub mod value;

// Core LLVM Types and Enumerations, code taken mostly from the Rust
// Project (src/librustc_llvm/lib.rs)

/* Opaque types. */

/**
 * The top-level container for all LLVM global data. See the LLVMContext class.
 */
#[allow(missing_copy_implementations)]
pub enum Context_opaque {}
pub type ContextRef = *mut Context_opaque;

/**
 * The top-level container for all other LLVM Intermediate Representation (IR)
 * objects.
 */
#[allow(missing_copy_implementations)]
pub enum Module_opaque {}
pub type ModuleRef = *mut Module_opaque;

/**
 * Each value in the LLVM IR has a type, an LLVMTypeRef.
 */
#[allow(missing_copy_implementations)]
pub enum Type_opaque {}
pub type TypeRef = *mut Type_opaque;

/**
 * Represents an individual value in LLVM IR.
 */
#[allow(missing_copy_implementations)]
pub enum Value_opaque {}
pub type ValueRef = *mut Value_opaque;

/**
 * Represents a basic block of instructions in LLVM IR.
 */
#[allow(missing_copy_implementations)]
pub enum BasicBlock_opaque {}
pub type BasicBlockRef = *mut BasicBlock_opaque;

/**
 * Represents an LLVM basic block builder.
 */
#[allow(missing_copy_implementations)]
pub enum Builder_opaque {}
pub type BuilderRef = *mut Builder_opaque;

#[allow(missing_copy_implementations)]
pub enum PassManager_opaque {}
pub type PassManagerRef = *mut PassManager_opaque;

#[allow(missing_copy_implementations)]
pub enum PassRegistry_opaque {}
pub type PassRegistryRef = *mut PassRegistry_opaque;

/**
 * Used to get the users and usees of a Value.
 */
#[allow(missing_copy_implementations)]
pub enum Use_opaque {}
pub type UseRef = *mut Use_opaque;

#[allow(missing_copy_implementations)]
pub enum DiagnosticInfo_opaque {}
pub type DiagnosticInfoRef = *mut DiagnosticInfo_opaque;

bitflags! {
    flags Attribute : u32 {
        const ZExtAttribute = 1 << 0,
        const SExtAttribute = 1 << 1,
        const NoReturnAttribute = 1 << 2,
        const InRegAttribute = 1 << 3,
        const StructRetAttribute = 1 << 4,
        const NoUnwindAttribute = 1 << 5,
        const NoAliasAttribute = 1 << 6,
        const ByValAttribute = 1 << 7,
        const NestAttribute = 1 << 8,
        const ReadNoneAttribute = 1 << 9,
        const ReadOnlyAttribute = 1 << 10,
        const NoInlineAttribute = 1 << 11,
        const AlwaysInlineAttribute = 1 << 12,
        const OptimizeForSizeAttribute = 1 << 13,
        const StackProtectAttribute = 1 << 14,
        const StackProtectReqAttribute = 1 << 15,
        const AlignmentAttribute = 31 << 16,
        const NoCaptureAttribute = 1 << 21,
        const NoRedZoneAttribute = 1 << 22,
        const NoImplicitFloatAttribute = 1 << 23,
        const NakedAttribute = 1 << 24,
        const InlineHintAttribute = 1 << 25,
        const StackAttribute = 7 << 26,
        const ReturnsTwiceAttribute = 1 << 29,
        const UWTableAttribute = 1 << 30,
        const NonLazyBindAttribute = 1 << 31,
    }
}

/**
* TODO: LLVM has an enum for instruction opcodes
*/
pub type Opcode = u32;

#[derive(Copy, PartialEq, Debug)]
#[repr(C)]
pub enum TypeKind {
    Void      = 0,  //**< type with no size */
    Half      = 1,  //**< 16 bit floating point type */
    Float     = 2,  //**< 32 bit floating point type */
    Double    = 3,  //**< 64 bit floating point type */
    X86_FP80  = 4,  //**< 80 bit floating point type (X87) */
    FP128     = 5,  //**< 128 bit floating point type (112-bit mantissa)*/
    PPC_FP128 = 6,  //**< 128 bit floating point type (two 64-bits) */
    Label     = 7,  //**< Labels */
    Integer   = 8,  //**< Arbitrary bit width integers */
    Function  = 9,  //**< Functions */
    Struct    = 10, //**< Structures */
    Array     = 11, //**< Arrays */
    Pointer   = 12, //**< Pointers */
    Vector    = 13, //**< SIMD 'packed' format, or other vector type */
    Metadata  = 14, //**< Metadata */
    X86_MMX   = 15, //**< X86 MMX */
}

// This enum omits the obsolete (and no-op) linkage types DLLImportLinkage,
// DLLExportLinkage, GhostLinkage and LinkOnceODRAutoHideLinkage.
// LinkerPrivateLinkage and LinkerPrivateWeakLinkage are not included either;
// they've been removed in upstream LLVM commit r203866.
#[derive(Copy)]
pub enum Linkage {
    ExternalLinkage = 0,            //**< Externally visible function */
    AvailableExternallyLinkage = 1,
    LinkOnceAnyLinkage = 2,         //**< Keep one copy of function when linking (inline)*/
    LinkOnceODRLinkage = 3,         //**< Same, but only replaced by something equivalent. */
    WeakAnyLinkage = 5,             //**< Keep one copy of function when linking (weak) */
    WeakODRLinkage = 6,             //**< Same, but only replaced by something equivalent. */
    AppendingLinkage = 7,           //**< Special purpose, only applies to global arrays */
    InternalLinkage = 8,            //**< Rename collisions when linking (static functions) */
    PrivateLinkage = 9,             //**< Like Internal, but omit from symbol table */
    ExternalWeakLinkage = 12,       //**< ExternalWeak linkage description */
    CommonLinkage = 14,             //**< Tentative definitions */
}

#[derive(Copy)]
pub enum Visibility {
    DefaultVisibility = 0,   //**< The GV is visible */
    HiddenVisibility = 1,    //**< The GV is hidden */
    ProtectedVisibility = 2, //**< The GV is protected */
}

#[derive(Copy)]
pub enum DLLStorageClass {
    DefaultStorageClass = 0,
    DLLImportStorageClass = 1, //**< Function to be imported from DLL. */
    DLLExportStorageClass = 2, //**< Function to be accessible from DLL. */
}

#[derive(Copy, PartialEq)]
pub enum CallConv {
    CCallConv = 0,
    FastCallConv = 8,
    ColdCallConv = 9,
    WebKitJSCallConv = 12,
    AnyRegCallConv = 13,
    X86StdcallCallConv = 64,
    X86FastcallCallConv = 65,
    X86_64_Win64 = 79,
}

#[derive(Copy)]
pub enum IntPredicate {
    IntEQ = 32,  //**< equal */
    IntNE = 33,  //**< not equal */
    IntUGT = 34, //**< unsigned greater than */
    IntUGE = 35, //**< unsigned greater or equal */
    IntULT = 36, //**< unsigned less than */
    IntULE = 37, //**< unsigned less or equal */
    IntSGT = 38, //**< signed greater than */
    IntSGE = 39, //**< signed greater or equal */
    IntSLT = 40, //**< signed less than */
    IntSLE = 41, //**< signed less or equal */
}

#[derive(Copy)]
pub enum RealPredicate {
    RealPredicateFalse = 0, //**< Always false (always folded) */
    RealOEQ = 1,            //**< True if ordered and equal */
    RealOGT = 2,            //**< True if ordered and greater than */
    RealOGE = 3,            //**< True if ordered and greater than or equal */
    RealOLT = 4,            //**< True if ordered and less than */
    RealOLE = 5,            //**< True if ordered and less than or equal */
    RealONE = 6,            //**< True if ordered and operands are unequal */
    RealORD = 7,            //**< True if ordered (no nans) */
    RealUNO = 8,            //**< True if unordered: isnan(X) | isnan(Y) */
    RealUEQ = 9,            //**< True if unordered or equal */
    RealUGT = 10,           //**< True if unordered or greater than */
    RealUGE = 11,           //**< True if unordered, greater than, or equal */
    RealULT = 12,           //**< True if unordered or less than */
    RealULE = 13,           //**< True if unordered, less than, or equal */
    RealUNE = 14,           //**< True if unordered or not equal */
    RealPredicateTrue = 15, //**< Always true (always folded) */
}

#[derive(Copy)]
pub enum LandingPadClauseTy {
    LandingPadCatch = 0,  //**< A catch clause   */
    LandingPadFilter = 1, //**< A filter clause  */
}

#[derive(Copy)]
pub enum ThreadLocalMode {
    NotThreadLocal = 0,
    GeneralDynamicTLSModel = 1,
    LocalDynamicTLSModel = 2,
    InitialExecTLSModel = 3,
    LocalExecTLSModel = 4,
}

#[repr(C)]
#[derive(Copy)]
pub enum AtomicOrdering {
    NotAtomic = 0,            //**< A load or store which is not atomic */
    Unordered = 1,            //**< Lowest level of atomicity, guarantees
                              //       somewhat sane results, lock free. */
    Monotonic = 2,            //**< guarantees that if you take all the
                              //       operations affecting a specific address,
                              //       a consistent ordering exists */
    // Consume = 3,           // Not specified yet.
    Acquire = 4,              //**< Acquire provides a barrier of the sort
                              //     necessary to acquire a lock to access other
                              //     memory with normal loads and stores. */
    Release = 5,              //**< Release is similar to Acquire, but with
                              //     a barrier of the sort necessary to release
                              //     a lock. */
    AcquireRelease = 6,       //**< provides both an Acquire and a
                              //            Release barrier (for fences and
                              //            operations which both read and write
                              //             memory). */
    SequentiallyConsistent = 7 //**< provides Acquire semantics
                               //                  for loads and Release
                               //                  semantics for stores.
                               //                  Additionally, it guarantees
                               //                  that a total ordering exists
                               //                  between all
                               //                  SequentiallyConsistent
                               //                  operations. */
}

#[repr(C)]
#[derive(Copy)]
pub enum AtomicRMWBinOp {
    AtomicRMWBinOpXchg = 0,  //**< Set the new value and return the one old */
    AtomicRMWBinOpAdd = 1,   //**< Add a value and return the old one */
    AtomicRMWBinOpSub = 2,   //**< Subtract a value and return the old one */
    AtomicRMWBinOpAnd = 3,   //**< And a value and return the old one */
    AtomicRMWBinOpNand = 4,  //**< Not-And a value and return the old one */
    AtomicRMWBinOpOr = 5,    //**< OR a value and return the old one */
    AtomicRMWBinOpXor = 6,   //**< Xor a value and return the old one */
    AtomicRMWBinOpMax = 7,   //**< Sets the value if it's greater than the
                             //     original using a signed comparison and return
                             //     the old one */
    AtomicRMWBinOpMin = 8,   //**< Sets the value if it's Smaller than the
                             //     original using a signed comparison and return
                             //     the old one */
    AtomicRMWBinOpUMax = 9,  //**< Sets the value if it's greater than the
                             //     original using an unsigned comparison and return
                              //    the old one */
    AtomicRMWBinOpUMin  = 10, //**< Sets the value if it's greater than the
                              //     original using an unsigned comparison  and return
                              //     the old one */

}

#[repr(C)]
#[derive(Copy, Debug)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Remark,
    Note,
}

pub type DiagnosticHandler = unsafe extern "C" fn(DiagnosticInfoRef, *mut c_void);
pub type YieldCallback = unsafe extern "C" fn(ContextRef, *mut c_void);

pub mod ffi {
    use libc::c_char;

    #[link(name = "LLVMCore")]
    extern {
        pub fn LLVMCreateMessage(Message: *const c_char) -> *const c_char;

        pub fn LLVMDisposeMessage(Message: *const c_char);
    }
}
