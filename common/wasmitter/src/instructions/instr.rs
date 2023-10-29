use std::fmt::{self, Display, Formatter};

use crate::{
    error::WasmError,
    func::Func,
    indices::{FuncIdx, GlobalIdx, LabelIdx, LocalIdx, TypeIdx, WasmIndex},
    module::Module,
    types::{Expr, FuncType, ResultType, ValType},
};

#[derive(Clone, Copy, Debug)]
pub enum Nn {
    N32,
    N64,
}

impl Display for Nn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Nn::N32 => "32",
                Nn::N64 => "64",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Sx {
    U,
    S,
}

impl Display for Sx {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sx::U => "u",
                Sx::S => "s",
            }
        )
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct MemArg<const N: usize> {
    offset: u32,
    align: u32,
}

impl<const N: usize> Display for MemArg<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.offset != 0 {
            write!(f, " offset={}", self.offset)?;
        }
        if self.align != 0 {
            write!(f, " align={}", self.align)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum BlockType {
    Type(TypeIdx),
    Val(Option<ValType>),
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Val(None)
    }
}

impl BlockType {
    fn emit_wat_inline(&self, module: &Module) -> String {
        let func_type = match self {
            BlockType::Type(type_idx) => module.get_signature(*type_idx).clone(),
            BlockType::Val(val_type) => FuncType {
                params: ResultType(Vec::new()),
                results: ResultType(match val_type {
                    Some(val_type) => vec![val_type.clone()],
                    None => Vec::new(),
                }),
            },
        };

        func_type.emit_wat_inline()
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Instr {
    /// `i32.const u32`
    I32Const(u32),
    /// `i64.const u64`
    I64Const(u64),
    /// `f32.const f32`
    F32Const(f32),
    /// `f64.const f64`
    F64Const(f64),
    /// `inn.clz`
    IClz(Nn),
    /// `inn.ctz`
    ICtz(Nn),
    /// `inn.popcnt`
    IPopcnt(Nn),
    /// `fnn.abs`
    FAbs(Nn),
    /// `fnn.neg`
    FNeg(Nn),
    /// `fnn.sqrt`
    FSqrt(Nn),
    /// `fnn.ceil`
    FCeil(Nn),
    /// `fnn.floor`
    FFloor(Nn),
    /// `fnn.trunc`
    FTrunc(Nn),
    /// `fnn.nearest`
    FNearest(Nn),
    /// `inn.add`
    IAdd(Nn),
    /// `inn.sub`
    ISub(Nn),
    /// `inn.mul`
    IMul(Nn),
    /// `inn.div_sx`
    IDiv(Nn, Sx),
    /// `inn.rem_sx`
    IRem(Nn, Sx),
    /// `inn.and`
    IAnd(Nn),
    /// `inn.or`
    IOr(Nn),
    /// `inn.xor`
    IXor(Nn),
    /// `inn.shl`
    IShl(Nn),
    /// `inn.shr_sx`
    IShr(Nn, Sx),
    /// `inn.rotl`
    IRotl(Nn),
    /// `inn.rotr`
    IRotr(Nn),
    /// `fnn.add`
    FAdd(Nn),
    /// `fnn.sub`
    FSub(Nn),
    /// `fnn.mul`
    FMul(Nn),
    /// `fnn.div`
    FDiv(Nn),
    /// `fnn.min`
    FMin(Nn),
    /// `fnn.max`
    FMax(Nn),
    /// `fnn.copysign`
    FCopysign(Nn),
    /// `inn.eqz`
    IEqz(Nn),
    /// `inn.eq`
    IEq(Nn),
    /// `inn.ne`
    INe(Nn),
    /// `inn.lt_sx`
    ILt(Nn, Sx),
    /// `inn.gt_sx`
    IGt(Nn, Sx),
    /// `inn.le_sx`
    ILe(Nn, Sx),
    /// `inn.ge_sx`
    IGe(Nn, Sx),
    /// `fnn.eq`
    FEq(Nn),
    /// `fnn.ne`
    FNe(Nn),
    /// `fnn.lt`
    FLt(Nn),
    /// `fnn.gt`
    FGt(Nn),
    /// `fnn.le`
    FLe(Nn),
    /// `fnn.ge`
    FGe(Nn),
    /// `inn.extend8_s`
    IExtend8S(Nn),
    /// `inn.extend16_s`
    IExtend16S(Nn),
    /// `i64.extend32_s`
    I64Extend32S,
    /// `i32.wrap_i64`
    I32WrapI64,
    /// `i64.extend_i32_sx`
    I64ExtendI32(Sx),
    /// `inn.trunc_fmm_sx`
    ITruncF(Nn, Nn, Sx),
    /// `inn.trunc_sat_fmm_sx`
    ITruncSatF(Nn, Nn, Sx),
    /// `f32.demote_f64`
    F32DemoteF64,
    /// `f64.promote_f32`
    F64PromoteF32,
    /// `fnn.convert_imm_sx`
    FConvertI(Nn, Nn, Sx),
    /// `inn.reinterpret_fnn`
    IReinterpretF(Nn),
    /// `fnn.reinterpret_inn`
    FReinterpretI(Nn),
    /// `drop`
    Drop,
    /// `select`
    Select,
    /// `local.get localidx`
    LocalGet(LocalIdx),
    /// `local.set localidx`
    LocalSet(LocalIdx),
    /// `local.tee localidx`
    LocalTee(LocalIdx),
    /// `global.get globalidx`
    GlobalGet(GlobalIdx),
    /// `global.set globalidx`
    GlobalSet(GlobalIdx),
    /// `i32.load memarg4`
    I32Load(MemArg<4>),
    /// `i64.load memarg8`
    I64Load(MemArg<8>),
    /// `f32.load memarg4`
    F32Load(MemArg<4>),
    /// `f64.load memarg8`
    F64Load(MemArg<8>),
    /// `i32.store memarg4`
    I32Store(MemArg<4>),
    /// `i64.store memarg8`
    I64Store(MemArg<8>),
    /// `f32.store memarg4`
    F32Store(MemArg<4>),
    /// `f64.store memarg8`
    F64Store(MemArg<8>),
    /// `inn.load8_sx memarg1`
    ILoad8(Nn, Sx, MemArg<1>),
    /// `inn.load16_sx memarg2`
    ILoad16(Nn, Sx, MemArg<2>),
    /// `i64.load32_sx memarg4`
    I64Load32(Sx, MemArg<4>),
    /// `inn.store8 memarg1`
    IStore8(Nn, MemArg<1>),
    /// `inn.store16 memarg2`
    IStore16(Nn, MemArg<2>),
    /// `i64.store32 memarg4`
    I64Store32(MemArg<4>),
    /// `memory.size`
    MemorySize,
    /// `memory.grow`
    MemoryGrow,
    /// `memory.fill`
    MemoryFill,
    /// `memory.copy`
    MemoryCopy,
    /// `nop`
    Nop,
    /// `unreachable`
    Unreachable,
    /// `block blocktype instr* end`
    Block(BlockType, Vec<Instr>),
    /// `loop blocktype instr* end`
    Loop(BlockType, Vec<Instr>),
    /// `br labelidx`
    Br(LabelIdx),
    /// `br_if labelidx`
    BrIf(LabelIdx),
    /// `return`
    Return,
    /// `call funcidx`
    Call(FuncIdx),
}

impl Instr {
    pub(crate) fn validate(&self, module: &Module, func: &Func) -> Option<WasmError> {
        match self {
            Instr::LocalGet(idx) | Instr::LocalSet(idx) | Instr::LocalTee(idx) =>
                if idx.belongs_to((module, func)) {
                    None
                } else {
                    Some(WasmError::FuncMismatch)
                },
            Instr::GlobalGet(idx) | Instr::GlobalSet(idx) =>
                if idx.belongs_to(module) {
                    None
                } else {
                    Some(WasmError::ModuleMismatch)
                },
            Instr::Block(_, instrs) | Instr::Loop(_, instrs) => instrs
                .iter()
                .flat_map(|instr| instr.validate(module, func))
                .next(),
            Instr::Call(idx) =>
                if idx.belongs_to(module) {
                    None
                } else {
                    Some(WasmError::ModuleMismatch)
                },
            _ => None,
        }
    }

    pub(crate) fn emit_wat_block(&self, module: &Module, func: &Func, indent: usize) -> String {
        let tab = " ".repeat(indent);
        format!(
            "{tab}({})\n",
            match self {
                Instr::I32Const(val) => format!("i32.const {}", *val as i32),
                Instr::I64Const(val) => format!("i64.const {}", *val as i64),
                Instr::F32Const(val) => format!("f32.const {val}"),
                Instr::F64Const(val) => format!("f64.const {val}"),
                Instr::IClz(nn) => format!("i{nn}.clz"),
                Instr::ICtz(nn) => format!("i{nn}.ctz"),
                Instr::IPopcnt(nn) => format!("i{nn}.popcnt"),
                Instr::FAbs(nn) => format!("f{nn}.abs"),
                Instr::FNeg(nn) => format!("f{nn}.neg"),
                Instr::FSqrt(nn) => format!("f{nn}.sqrt"),
                Instr::FCeil(nn) => format!("f{nn}.ceil"),
                Instr::FFloor(nn) => format!("f{nn}.floor"),
                Instr::FTrunc(nn) => format!("f{nn}.trunc"),
                Instr::FNearest(nn) => format!("f{nn}.nearest"),
                Instr::IAdd(nn) => format!("i{nn}.add"),
                Instr::ISub(nn) => format!("i{nn}.sub"),
                Instr::IMul(nn) => format!("i{nn}.mul"),
                Instr::IDiv(nn, sx) => format!("i{nn}.div_{sx}"),
                Instr::IRem(nn, sx) => format!("i{nn}.rem_{sx}"),
                Instr::IAnd(nn) => format!("i{nn}.and"),
                Instr::IOr(nn) => format!("i{nn}.or"),
                Instr::IXor(nn) => format!("i{nn}.xor"),
                Instr::IShl(nn) => format!("i{nn}.shl"),
                Instr::IShr(nn, sx) => format!("i{nn}.shr_{sx}"),
                Instr::IRotl(nn) => format!("i{nn}.rotl"),
                Instr::IRotr(nn) => format!("i{nn}.rotr"),
                Instr::FAdd(nn) => format!("f{nn}.add"),
                Instr::FSub(nn) => format!("f{nn}.sub"),
                Instr::FMul(nn) => format!("f{nn}.mul"),
                Instr::FDiv(nn) => format!("f{nn}.div"),
                Instr::FMin(nn) => format!("f{nn}.min"),
                Instr::FMax(nn) => format!("f{nn}.max"),
                Instr::FCopysign(nn) => format!("f{nn}.copysign"),
                Instr::IEqz(nn) => format!("i{nn}.eqz"),
                Instr::IEq(nn) => format!("i{nn}.eq"),
                Instr::INe(nn) => format!("i{nn}.ne"),
                Instr::ILt(nn, sx) => format!("i{nn}.lt_{sx}"),
                Instr::IGt(nn, sx) => format!("i{nn}.gt_{sx}"),
                Instr::ILe(nn, sx) => format!("i{nn}.le_{sx}"),
                Instr::IGe(nn, sx) => format!("i{nn}.ge_{sx}"),
                Instr::FEq(nn) => format!("f{nn}.eq"),
                Instr::FNe(nn) => format!("f{nn}.ne"),
                Instr::FLt(nn) => format!("f{nn}.lt"),
                Instr::FGt(nn) => format!("f{nn}.gt"),
                Instr::FLe(nn) => format!("f{nn}.le"),
                Instr::FGe(nn) => format!("f{nn}.ge"),
                Instr::IExtend8S(nn) => format!("i{nn}.extend8_s"),
                Instr::IExtend16S(nn) => format!("i{nn}.extend16_s"),
                Instr::I64Extend32S => "i64.extend32_s".into(),
                Instr::I32WrapI64 => "i32.wrap_i64".into(),
                Instr::I64ExtendI32(sx) => format!("i64.extend_i32_{sx}"),
                Instr::ITruncF(nn, mm, sx) => format!("i{nn}.trunc_f{mm}_{sx}"),
                Instr::ITruncSatF(nn, mm, sx) => format!("i{nn}.trunc_sat_f{mm}_{sx}"),
                Instr::F32DemoteF64 => "f32.demote_f64".into(),
                Instr::F64PromoteF32 => "f64.promote_f32".into(),
                Instr::FConvertI(nn, mm, sx) => format!("f{nn}.convert_i{mm}_{sx}"),
                Instr::IReinterpretF(nn) => format!("i{nn}.reinterpret_f{nn}"),
                Instr::FReinterpretI(nn) => format!("f{nn}.reinterpret_i{nn}"),
                Instr::Drop => "drop".into(),
                Instr::Select => "select".into(),
                Instr::LocalGet(idx) => format!("local.get {}", idx.id_or_index((module, func))),
                Instr::LocalSet(idx) => format!("local.set {}", idx.id_or_index((module, func))),
                Instr::LocalTee(idx) => format!("local.tee {}", idx.id_or_index((module, func))),
                Instr::GlobalGet(idx) => format!("global.get {}", idx.id_or_index(module)),
                Instr::GlobalSet(idx) => format!("global.set {}", idx.id_or_index(module)),
                Instr::I32Load(memarg) => format!("i32.load{memarg}"),
                Instr::I64Load(memarg) => format!("i64.load{memarg}"),
                Instr::F32Load(memarg) => format!("f32.load{memarg}"),
                Instr::F64Load(memarg) => format!("f64.load{memarg}"),
                Instr::I32Store(memarg) => format!("i32.store{memarg}"),
                Instr::I64Store(memarg) => format!("i64.store{memarg}"),
                Instr::F32Store(memarg) => format!("f32.store{memarg}"),
                Instr::F64Store(memarg) => format!("f64.store{memarg}"),
                Instr::ILoad8(nn, sx, memarg) => format!("i{nn}.load8_{sx}{memarg}"),
                Instr::ILoad16(nn, sx, memarg) => format!("i{nn}.load16_{sx}{memarg}"),
                Instr::I64Load32(sx, memarg) => format!("i64.load32_{sx}{memarg}"),
                Instr::IStore8(nn, memarg) => format!("i{nn}.store8{memarg}"),
                Instr::IStore16(nn, memarg) => format!("i{nn}.store16{memarg}"),
                Instr::I64Store32(memarg) => format!("i64.store32{memarg}"),
                Instr::MemorySize => "memory.size".into(),
                Instr::MemoryGrow => "memory.grow".into(),
                Instr::MemoryFill => "memory.fill".into(),
                Instr::MemoryCopy => "memory.copy".into(),
                Instr::Nop => "nop".into(),
                Instr::Unreachable => "unreachable".into(),
                Instr::Block(block_type, instrs) => format!(
                    "block {}\n{}{tab}",
                    block_type.emit_wat_inline(module),
                    Expr(instrs.clone()).emit_wat_block(module, func, indent + 2)
                ),
                Instr::Loop(block_type, instrs) => format!(
                    "loop {}\n{}{tab}",
                    block_type.emit_wat_inline(module),
                    Expr(instrs.clone()).emit_wat_block(module, func, indent + 2)
                ),
                Instr::Br(idx) => format!("br {}", idx.id_or_index(())),
                Instr::BrIf(idx) => format!("br_if {}", idx.id_or_index(())),
                Instr::Return => "return".into(),
                Instr::Call(idx) => format!("call {}", idx.id_or_index(module)),
            }
        )
    }
}
