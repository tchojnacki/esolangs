use crate::{
    indices::{FuncIdx, GlobalIdx, MemIdx, TypeIdx, WasmIndex},
    instruction::Instr,
    module::Module,
};

#[derive(Clone, Copy, Debug)]
pub struct Id(Option<&'static str>);

impl Id {
    pub fn none() -> Self {
        Self(None)
    }

    pub(crate) fn into_option(self) -> Option<&'static str> {
        self.0
    }
}

impl From<&'static str> for Id {
    fn from(alias: &'static str) -> Self {
        Self(Some(alias))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl NumType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            NumType::I32 => "i32",
            NumType::I64 => "i64",
            NumType::F32 => "f32",
            NumType::F64 => "f64",
        }
        .into()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct VecType;

#[derive(PartialEq, Clone, Debug)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}

impl ValType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            ValType::Num(num) => num.emit_wat_inline(),
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ResultType(pub(crate) Vec<ValType>);

impl From<ValType> for ResultType {
    fn from(val_type: ValType) -> Self {
        Self(vec![val_type])
    }
}

impl From<Vec<ValType>> for ResultType {
    fn from(val_types: Vec<ValType>) -> Self {
        Self(val_types)
    }
}

impl From<()> for ResultType {
    fn from(_: ()) -> Self {
        Self(Vec::new())
    }
}

impl ResultType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        self.0
            .iter()
            .map(|t| t.emit_wat_inline())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FuncType {
    pub params: ResultType,
    pub results: ResultType,
}

impl FuncType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        let construct_part = |result_type: &ResultType, name: &str| {
            let wat = result_type.emit_wat_inline();
            if wat.is_empty() {
                String::new()
            } else {
                format!("({name} {wat})")
            }
        };

        [
            construct_part(&self.params, "param"),
            construct_part(&self.results, "result"),
        ]
        .iter()
        .cloned()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
    }
}

#[derive(Debug)]
pub(crate) struct Limits {
    pub(crate) min: u32,
    pub(crate) max: u32,
}

#[derive(Debug)]
pub struct MemType {
    pub(crate) limits: Limits,
}

#[derive(Debug)]
pub struct GlobalType {
    pub(crate) mutability: Mutability,
    pub(crate) val_type: ValType,
}

impl GlobalType {
    pub(crate) fn emit_wat_inline(&self) -> String {
        format!(
            "({} {})",
            self.mutability.emit_wat_inline(),
            self.val_type.emit_wat_inline()
        )
    }
}

#[derive(Debug)]
pub enum Mutability {
    Mut,
    Const,
}

impl Mutability {
    pub(crate) fn emit_wat_inline(&self) -> String {
        match self {
            Mutability::Mut => "mut",
            Mutability::Const => "const",
        }
        .into()
    }
}

#[derive(Clone, Debug)]
pub struct Expr(pub(crate) Vec<Instr>);

impl From<Vec<Instr>> for Expr {
    fn from(instrs: Vec<Instr>) -> Self {
        Self(instrs)
    }
}

impl From<Instr> for Expr {
    fn from(instr: Instr) -> Self {
        Self(vec![instr])
    }
}

impl Expr {
    pub(crate) fn emit_wat_block(&self, module: &Module, func: &Func, indent: usize) -> String {
        let mut result = String::new();
        let func = Some(func);
        for instr in &self.0 {
            result.push_str(&instr.emit_wat_block(module, func, indent));
        }
        result
    }
}

#[derive(Debug)]
pub(crate) struct Func {
    pub(crate) type_idx: TypeIdx,
    pub(crate) func_idx: FuncIdx,
    pub(crate) locals: Vec<ValType>,
    pub(crate) body: Expr,
}

impl Func {
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        let tab = " ".repeat(indent);
        let mut result = String::new();
        let func_type = module.get_signature(self.type_idx);

        result.push_str(&format!(
            "{tab}(func {} {}\n",
            self.func_idx.id_or_comment(module),
            func_type.emit_wat_inline()
        ));

        for local in &self.locals {
            result.push_str(&format!("{tab}  (local {})\n", local.emit_wat_inline()));
        }

        result.push_str(&self.body.emit_wat_block(module, self, indent + 2));

        result.push_str(&format!("{tab})\n"));

        result
    }
}

#[derive(Debug)]
pub struct Mem {
    pub(crate) mem_type: MemType,
    pub(crate) mem_idx: MemIdx,
}

impl Mem {
    pub(crate) fn emit_wat_block(&self, indent: usize) -> String {
        format!(
            "{}(memory {} {} {})\n",
            " ".repeat(indent),
            self.mem_idx.id_or_comment(()),
            self.mem_type.limits.min,
            self.mem_type.limits.max
        )
    }
}

#[derive(Debug)]
pub struct Global {
    pub(crate) global_type: GlobalType,
    pub(crate) init: Instr,
    pub(crate) global_idx: GlobalIdx,
}

impl Global {
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        let tab = " ".repeat(indent);
        format!(
            "{tab}(global {} {}\n{}{tab})\n",
            self.global_idx.id_or_comment(()),
            self.global_type.emit_wat_inline(),
            self.init.emit_wat_block(module, None, indent + 2),
        )
    }
}

#[derive(Debug)]
pub enum ImportDesc {
    Func {
        type_idx: TypeIdx,
        func_idx: FuncIdx,
    },
    Mem(MemType),
    Global(GlobalType),
}

impl ImportDesc {
    pub(crate) fn emit_wat_inline(&self, module: &Module) -> String {
        match self {
            ImportDesc::Func { type_idx, func_idx } => {
                let func_type = module.get_signature(*type_idx);
                let alias = func_idx.id_or_comment(module);
                format!("(func {} {})", alias, func_type.emit_wat_inline())
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

impl Import {
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(import \"{}\" \"{}\" {})\n",
            " ".repeat(indent),
            self.module,
            self.name,
            self.desc.emit_wat_inline(module)
        )
    }
}

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

impl From<FuncIdx> for ExportDesc {
    fn from(func_idx: FuncIdx) -> Self {
        Self::Func(func_idx)
    }
}

impl From<MemIdx> for ExportDesc {
    fn from(mem_idx: MemIdx) -> Self {
        Self::Mem(mem_idx)
    }
}

impl From<GlobalIdx> for ExportDesc {
    fn from(global_idx: GlobalIdx) -> Self {
        Self::Global(global_idx)
    }
}

impl ExportDesc {
    pub(crate) fn emit_wat_inline(&self, module: &Module) -> String {
        match self {
            ExportDesc::Func(func_idx) => format!("(func {})", func_idx.id_or_index(module)),
            ExportDesc::Mem(mem_idx) => format!("(memory {})", mem_idx.id_or_index(())),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Export {
    pub(crate) name: String,
    pub(crate) desc: ExportDesc,
}

impl Export {
    pub(crate) fn emit_wat_block(&self, module: &Module, indent: usize) -> String {
        format!(
            "{}(export \"{}\" {})\n",
            " ".repeat(indent),
            self.name,
            self.desc.emit_wat_inline(module)
        )
    }
}

pub const I32: ValType = ValType::Num(NumType::I32);
pub const I64: ValType = ValType::Num(NumType::I64);
pub const F32: ValType = ValType::Num(NumType::F32);
pub const F64: ValType = ValType::Num(NumType::F64);
