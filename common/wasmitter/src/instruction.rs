use std::fmt::{self, Display, Formatter};

use crate::{
    indices::{DataIdx, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, WasmIndex},
    module::Module,
    types::Func,
};

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Default, Debug)]
pub struct MemArg {
    offset: u32,
    align: u32,
}

#[derive(Debug)]
pub enum Ww {
    W8,
    W16,
    W32,
    W64,
}

#[derive(Debug)]
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
    /// `inn.load memarg`
    ILoad(Nn, MemArg),
    /// `fnn.load memarg`
    FLoad(Nn, MemArg),
    /// `inn.store memarg`
    IStore(Nn, MemArg),
    /// `fnn.store memarg`
    FStore(Nn, MemArg),
    /// `inn.load8_sx memarg`
    ILoad8(Nn, Sx, MemArg),
    /// `inn.load16_sx memarg`
    ILoad16(Nn, Sx, MemArg),
    /// `i64.load32_sx memarg`
    I64Load32(Sx, MemArg),
    /// `inn.store8 memarg`
    IStore8(Nn, MemArg),
    /// `inn.store16 memarg`
    IStore16(Nn, MemArg),
    /// `i64.store32 memarg`
    I64Store32(MemArg),
    // TODO: Vector instructions
    /// `memory.size`
    MemorySize,
    /// `memory.grow`
    MemoryGrow,
    /// `memory.fill`
    MemoryFill,
    /// `memory.copy`
    MemoryCopy,
    /// `memory.init dataidx`
    MemoryInit(DataIdx),
    /// `data.drop dataidx`
    DataDrop(DataIdx),
    /// `nop`
    Nop,
    /// `unreachable`
    Unreachable,
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
    pub(crate) fn emit_wat_inline(&self, module: &Module, func: Option<&Func>) -> String {
        let ctx = || {
            (
                module,
                func.expect("local instruction used outside of a function"),
            )
        };

        format!(
            "({})",
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
                Instr::LocalGet(idx) => format!("local.get {}", idx.alias_or_index(ctx())),
                Instr::LocalSet(idx) => format!("local.set {}", idx.alias_or_index(ctx())),
                Instr::LocalTee(idx) => format!("local.tee {}", idx.alias_or_index(ctx())),
                Instr::GlobalGet(idx) => format!("global.get {}", idx.alias_or_index(())),
                Instr::GlobalSet(idx) => format!("global.set {}", idx.alias_or_index(())),
                Instr::ILoad(nn, _) => format!("i{nn}.load"),
                Instr::FLoad(nn, _) => format!("f{nn}.load"),
                Instr::IStore(nn, _) => format!("i{nn}.store"),
                Instr::FStore(nn, _) => format!("f{nn}.store"),
                Instr::ILoad8(nn, sx, _) => format!("i{nn}.load8_{sx}"),
                Instr::ILoad16(nn, sx, _) => format!("i{nn}.load16_{sx}"),
                Instr::I64Load32(sx, _) => format!("i64.load32_{sx}"),
                Instr::IStore8(nn, _) => format!("i{nn}.store8"),
                Instr::IStore16(nn, _) => format!("i{nn}.store16"),
                Instr::I64Store32(_) => "i64.store32".into(),
                Instr::MemorySize => "memory.size".into(),
                Instr::MemoryGrow => "memory.grow".into(),
                Instr::MemoryFill => "memory.fill".into(),
                Instr::MemoryCopy => "memory.copy".into(),
                Instr::MemoryInit(idx) => format!("memory.init {}", idx.alias_or_index(())),
                Instr::DataDrop(idx) => format!("data.drop {}", idx.alias_or_index(())),
                Instr::Nop => "nop".into(),
                Instr::Unreachable => "unreachable".into(),
                Instr::Br(idx) => format!("br {}", idx.alias_or_index(())),
                Instr::BrIf(idx) => format!("br_if {}", idx.alias_or_index(())),
                Instr::Return => "return".into(),
                Instr::Call(idx) => format!("call {}", idx.alias_or_index(module)),
            }
        )
    }

    pub(crate) fn emit_wat_block(
        &self,
        module: &Module,
        func: Option<&Func>,
        indent: usize,
    ) -> String {
        format!(
            "{}{}\n",
            " ".repeat(indent),
            self.emit_wat_inline(module, func)
        )
    }
}