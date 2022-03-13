use crate::util::*;
use core::{fmt, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Opcode {
    Func,
    Loc,

    StackAlloc,
    StackDealloc,

    Make8,
    Make16,
    Make32,
    Make64,
    MakeFp,
    MakeSp,

    PushUndef,
    Pop,
    Swap,
    Dup,
    PushDyn,

    SExtend8To16,
    SExtend8To32,
    SExtend8To64,
    SExtend16To32,
    SExtend16To64,
    SExtend32To64,

    ZExtend8To16,
    ZExtend8To32,
    ZExtend8To64,
    ZExtend16To32,
    ZExtend16To64,
    ZExtend32To64,

    I8ToF32,
    U8ToF32,
    I8ToF64,
    U8ToF64,
    I16ToF32,
    U16ToF32,
    I16ToF64,
    U16ToF64,
    I32ToF32,
    U32ToF32,
    I32ToF64,
    U32ToF64,
    I64ToF32,
    U64ToF32,
    I64ToF64,
    U64ToF64,

    F32ToI8,
    F32ToU8,
    F64ToI8,
    F64ToU8,
    F32ToI16,
    F32ToU16,
    F64ToI16,
    F64ToU16,
    F32ToI32,
    F32ToU32,
    F64ToI32,
    F64ToU32,
    F32ToI64,
    F32ToU64,
    F64ToI64,
    F64ToU64,

    F32ToF64,
    F64ToF32,

    Get,
    Set,

    BoolNorm8,
    BoolNorm16,
    BoolNorm32,
    BoolNorm64,

    BoolNot8,
    BoolNot16,
    BoolNot32,
    BoolNot64,

    Add8,
    Add16,
    Add32,
    Add64,
    AddF32,
    AddF64,

    SubI8,
    SubU8,
    SubI16,
    SubU16,
    SubI32,
    SubU32,
    SubI64,
    SubU64,
    SubF32,
    SubF64,

    MulI8,
    MulU8,
    MulI16,
    MulU16,
    MulI32,
    MulU32,
    MulI64,
    MulU64,
    MulF32,
    MulF64,

    DivI8,
    DivU8,
    DivI16,
    DivU16,
    DivI32,
    DivU32,
    DivI64,
    DivU64,
    DivF32,
    DivF64,

    ModI8,
    ModU8,
    ModI16,
    ModU16,
    ModI32,
    ModU32,
    ModI64,
    ModU64,
    ModF32,
    ModF64,

    CompLtI8,
    CompLtU8,
    CompLtI16,
    CompLtU16,
    CompLtI32,
    CompLtU32,
    CompLtI64,
    CompLtU64,
    CompLtF32,
    CompLtF64,

    CompLeqI8,
    CompLeqU8,
    CompLeqI16,
    CompLeqU16,
    CompLeqI32,
    CompLeqU32,
    CompLeqI64,
    CompLeqU64,
    CompLeqF32,
    CompLeqF64,

    CompEq8,
    CompEq16,
    CompEq32,
    CompEq64,
    CompEqF32,
    CompEqF64,

    CompNeq8,
    CompNeq16,
    CompNeq32,
    CompNeq64,
    CompNeqF32,
    CompNeqF64,

    RShiftI8,
    RShiftU8,
    RShiftI16,
    RShiftU16,
    RShiftI32,
    RShiftU32,
    RShiftI64,
    RShiftU64,

    LShiftI8,
    LShiftU8,
    LShiftI16,
    LShiftU16,
    LShiftI32,
    LShiftU32,
    LShiftI64,
    LShiftU64,

    BitAnd8,
    BitAnd16,
    BitAnd32,
    BitAnd64,

    BitOr8,
    BitOr16,
    BitOr32,
    BitOr64,

    BitXor8,
    BitXor16,
    BitXor32,
    BitXor64,

    BitNot8,
    BitNot16,
    BitNot32,
    BitNot64,

    Jump,

    JumpIfZero8,
    JumpIfZero16,
    JumpIfZero32,
    JumpIfZero64,

    JumpIfNotZero8,
    JumpIfNotZero16,
    JumpIfNotZero32,
    JumpIfNotZero64,

    Ret,
    Call,

    AllocBegin,
    AllocEnd,
    HeapAlloc,
    HeapDealloc,

    Throw,

    Ecall,

    AssertStr,
}