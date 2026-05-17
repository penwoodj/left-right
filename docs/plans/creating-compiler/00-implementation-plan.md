# Left-Right Compiler (Bytecode VM) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a complete, production-ready Left-Right compiler that transpiles `.lr` source to bytecode for a register-based VM, with comprehensive testing coverage and optimized execution.

**Architecture:** AST → IR (SSA form) → Optimized IR → Bytecode → Register-based VM execution (32-bit fixed-width instructions, Lua-style)

**Tech Stack:** Rust (1.75+), gc-arena v0.5.3 (safe, incremental, exact, cycle-detecting GC), goldenfile v1.11.0 (golden tests), insta v1.47.2 (snapshots), ariadne v0.6.0 (diagnostics)

---

## Agent Skills Required

Throughout implementation, use these skills:
- **writing-plans**: For planning sub-tasks (already active)
- **test-driven-development**: For implementing features (write test first, then code)
- **systematic-debugging**: For fixing issues (3-5 bug reports → use this skill)
- **verification-before-completion**: For checking work before claiming done
- **commit**: For git commits (NEVER commit directly without this skill)

---

## Project Structure

Create this workspace structure:

```
/home/jon/code/left-right/
├── Cargo.toml                    # Workspace root
├── compiler/
│   ├── Cargo.toml               # Compiler workspace
│   ├── crates/
│   │   ├── lr-ast/              # AST types and parser
│   │   ├── lr-ir/               # Intermediate representation (SSA)
│   │   ├── lr-bytecode/         # Bytecode encoding
│   │   ├── lr-compiler/         # Main compiler (AST → Bytecode)
│   │   ├── lr-vm/               # Bytecode VM (register-based)
│   │   └── lr-runtime/          # Runtime library (built-in operators)
│   └── tests/                   # Golden tests
│       ├── bytecode_roundtrip.lr
│       ├── simple_math.lr
│       ├── curried_application.lr
│       ├── map_as_function.lr
│       ├── string_interpolation.lr
│       ├── list_operations.lr
│       ├── map_operations.lr
│       ├── error_handling.lr
│       ├── async_operations.lr
│       ├── import_export.lr
│       └── complex_operations.lr
├── docs/
│   ├── specs/                    # Language specifications
│   ├── brainstorms/              # Original brainstorm documents
│   ├── reports/                  # Research reports
│   └── translations/             # JS translations
└── README.md                     # Project README (exists)
```

---

## Part 1: Workspace Setup and Dependencies

### Task 1: Initialize Workspace Root

**Files:**
- Create: `/home/jon/code/left-right/Cargo.toml`

- [ ] **Step 1: Write workspace Cargo.toml**

```toml
[workspace]
members = [
    "compiler/crates/lr-ast",
    "compiler/crates/lr-ir",
    "compiler/crates/lr-bytecode",
    "compiler/crates/lr-compiler",
    "compiler/crates/lr-vm",
    "compiler/crates/lr-runtime",
]
resolver = "2"

[workspace.dependencies]
# Core dependencies
gc-arena = "0.5.3"
goldenfile = "1.11.0"
insta = "1.47.2"
ariadne = "0.6.0"

# Testing
proptest = "1.11.0"
criterion = "0.5"

# Development
thiserror = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

- [ ] **Step 2: Verify workspace structure**

Run: `cargo --manifest-path Cargo.toml check --workspace`
Expected: No errors, workspace recognizes all crates

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: initialize Cargo workspace for compiler"
```

---

### Task 2: Create lr-ast Crate (AST Types and Parser)

**Files:**
- Create: `compiler/crates/lr-ast/Cargo.toml`
- Create: `compiler/crates/lr-ast/src/lib.rs`
- Create: `compiler/crates/lr-ast/src/ast.rs`
- Create: `compiler/crates/lr-ast/src/parser.rs`

- [ ] **Step 1: Write lr-ast/Cargo.toml**

```toml
[package]
name = "lr-ast"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = { workspace = true }
ariadne = { workspace = true }

[dev-dependencies]
goldenfile = { workspace = true }
```

- [ ] **Step 2: Write lr-ast/src/ast.rs (AST definitions)**

**NOTE: The canonical AST definitions live in docs/plans/creating-lexer-and-ast/00-implementation-plan.md. This section provides compiler-specific context (bytecode mapping, type tags) but references the same types. When implementing, use a shared lr-ast crate.**

```rust
//! Left-Right AST Types
//!
//! This file defines the complete AST structure for Left-Right source files.
//! All operators are identifiers — no separate Operator token type.
//! Evaluation is strictly left-to-right with zero precedence.

use std::fmt;

/// 7 types in Left-Right: Operator, Map, List, String, Boolean, Number, Undefined
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Operator,
    Map,
    List,
    String,
    Boolean,
    Number,
    Undefined,
}

/// NumberLiteral: decimal only (no hex, binary, octal, scientific)
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub value: f64,
    pub raw: String,
}

/// StringLiteral: backtick-delimited, supports interpolation
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub parts: Vec<StringPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Interpolation(Box<Expression>),
}

/// BooleanLiteral
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
    pub raw: String,
}

/// UndefinedLiteral
#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedLiteral {
    pub raw: String,
}

/// ListLiteral
#[derive(Debug, Clone, PartialEq)]
pub struct ListLiteral {
    pub elements: Vec<Expression>,
}

/// MapLiteral: universal construct (functions, conditionals, control flow, data)
#[derive(Debug, Clone, PartialEq)]
pub struct MapLiteral {
    pub entries: Vec<MapEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapEntry {
    pub key: Expression,
    pub value: Expression,
    pub is_assignment: bool,      // true if key starts with alpha → creates variable
    pub is_expression_key: bool,  // true if key is expression (not alpha-starting identifier)
}

/// Identifier: universal token for ALL operators and named values
/// `+`, `@`, `!!!`, `!!!?`, `///`, `\\\`, `$@` are ALL identifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

/// LeftArg: `_<` — represents whole left argument value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftArg {
    pub raw: String,
}

/// RightArg: `_>` — represents explicit right argument value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightArg {
    pub raw: String,
}

/// Application: fundamental AST node (left-to-right curried evaluation)
#[derive(Debug, Clone, PartialEq)]
pub struct Application {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

/// GroupedExpression: parentheses force evaluation order
#[derive(Debug, Clone, PartialEq)]
pub struct GroupedExpression {
    pub expression: Box<Expression>,
}

/// ImportExpression: imports@`pkg` or files@`path` (expressions, not keywords)
#[derive(Debug, Clone, PartialEq)]
pub struct ImportExpression {
    pub source: Box<Expression>,  // Identifier("imports") or Identifier("files")
    pub path: Box<Expression>,    // StringLiteral with module path
}

/// ExportExpression: }@&[`name1`, `name2`] (expression at end of file)
#[derive(Debug, Clone, PartialEq)]
pub struct ExportExpression {
    pub names: Box<ListLiteral>,
}

/// ThrowExpression: expr !!! (identifier, not keyword)
#[derive(Debug, Clone, PartialEq)]
pub struct ThrowExpression {
    pub value: Box<Expression>,
}

/// CatchExpression: operator !!!? handler (identifier, not keyword)
#[derive(Debug, Clone, PartialEq)]
pub struct CatchExpression {
    pub operator: Box<Expression>,
    pub handler: Box<Expression>,
}

/// AsyncExpression: operator /// (identifier, not keyword)
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncExpression {
    pub operator: Box<Expression>,
}

/// AwaitExpression: promise \\\ (identifier, not keyword)
#[derive(Debug, Clone, PartialEq)]
pub struct AwaitExpression {
    pub promise: Box<Expression>,
}

/// Program: single root expression (entire .lr file)
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub expression: Box<Expression>,
    pub source_path: String,
}

/// Universal Expression type
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    UndefinedLiteral(UndefinedLiteral),
    ListLiteral(ListLiteral),
    MapLiteral(MapLiteral),
    Identifier(Identifier),
    LeftArg(LeftArg),
    RightArg(RightArg),
    Application(Application),
    GroupedExpression(GroupedExpression),
    ImportExpression(ImportExpression),
    ExportExpression(ExportExpression),
    ThrowExpression(ThrowExpression),
    CatchExpression(CatchExpression),
    AsyncExpression(AsyncExpression),
    AwaitExpression(AwaitExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::NumberLiteral(n) => write!(f, "{}", n.raw),
            Expression::StringLiteral(s) => {
                write!(f, "`")?;
                for part in &s.parts {
                    match part {
                        StringPart::Text(text) => write!(f, "{}", text),
                        StringPart::Interpolation(expr) => write!(f, "{{{}}}", expr),
                    }?
                }
                write!(f, "`")
            }
            Expression::BooleanLiteral(b) => write!(f, "{}", b.raw),
            Expression::UndefinedLiteral(u) => write!(f, "{}", u.raw),
            Expression::ListLiteral(l) => {
                write!(f, "[")?;
                for (i, elem) in l.elements.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Expression::MapLiteral(m) => {
                write!(f, "{{ ")?;
                for (i, entry) in m.entries.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", entry.key, entry.value)?;
                }
                write!(f, " }}")
            }
            Expression::Identifier(id) => write!(f, "{}", id.name),
            Expression::LeftArg(l) => write!(f, "{}", l.raw),
            Expression::RightArg(r) => write!(f, "{}", r.raw),
            Expression::Application(app) => write!(f, "({} {})", app.left, app.right),
            Expression::GroupedExpression(g) => write!(f, "({})", g.expression),
            Expression::ImportExpression(ie) => write!(f, "{} @ {}", ie.source, ie.path),
            Expression::ExportExpression(ee) => write!(f, "}}@&{}", ee.names),
            Expression::ThrowExpression(te) => write!(f, "{} !!!", te.value),
            Expression::CatchExpression(ce) => write!(f, "{} !!!? {}", ce.operator, ce.handler),
            Expression::AsyncExpression(ae) => write!(f, "{} ///", ae.operator),
            Expression::AwaitExpression(ae) => write!(f, "{} \\\\", ae.promise),
        }
    }
}
```

- [ ] **Step 3: Write lr-ast/src/lib.rs**

```rust
pub mod ast;
pub mod parser;

pub use ast::*;
pub use parser::Parser;
```

- [ ] **Step 4: Run cargo check**

Run: `cargo check -p lr-ast`
Expected: No errors, crate compiles

- [ ] **Step 5: Commit**

```bash
git add compiler/crates/lr-ast/
git commit -m "feat: add lr-ast crate with AST type definitions"
```

---

### Task 3: Create lr-bytecode Crate (Bytecode Encoding)

**Files:**
- Create: `compiler/crates/lr-bytecode/Cargo.toml`
- Create: `compiler/crates/lr-bytecode/src/lib.rs`
- Create: `compiler/crates/lr-bytecode/src/instruction.rs`

- [ ] **Step 1: Write lr-bytecode/Cargo.toml**

```toml
[package]
name = "lr-bytecode"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = { workspace = true }

[dev-dependencies]
goldenfile = { workspace = true }
proptest = { workspace = true }
```

- [ ] **Step 2: Write lr-bytecode/src/instruction.rs**

```rust
//! Left-Right Bytecode Instruction Encoding
//!
//! 32-bit fixed-width instructions (Lua-style):
//! - 8-bit opcode (0-255)
//! - 3 x 8-bit register operands (A, B, C)
//!
//! Total: 256 opcodes, 256 registers each
//!
//! Reference: Lua VM uses 32-bit fixed-width instructions, ~47% fewer dispatches vs stack-based
//! https://luaf.dev/pages/virtualmachine.html
//!
//! **Design decision**: 32-bit fixed-width encoding chosen for simplicity and predictable dispatch.
//! Variable-width (VaryingOperand/LEB128) deferred to optimization phase. Lua/LuaJIT use 32-bit successfully.

use std::fmt;

/// 32-bit fixed-width instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction(pub u32);

impl Instruction {
    /// Create instruction from opcode and operands
    pub fn new(opcode: u8, a: u8, b: u8, c: u8) -> Self {
        let opcode = opcode as u32;
        let a = a as u32;
        let b = b as u32;
        let c = c as u32;
        Instruction((opcode << 24) | (a << 16) | (b << 8) | c)
    }

    pub fn opcode(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn a(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn b(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn c(self) -> u8 {
        self.0 as u8
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} r{}, r{}, r{}", self.opcode(), self.a(), self.b(), self.c())
    }
}

/// Opcodes for Left-Right bytecode VM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Opcode {
    // Control flow
    Nop = 0,
    Return,
    Jump,
    JumpIfTrue,
    JumpIfFalse,
    Call,
    TailCall,
    
    // Stack operations
    Push,
    Pop,
    Dup,
    
    // Load/Store
    LoadConstant,
    LoadRegister,
    StoreRegister,
    LoadLocal,
    StoreLocal,
    
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    
    // Boolean
    Not,
    And,
    Or,
    ToBoolean,
    
    // Map operations
    MapNew,
    MapGet,
    MapSet,
    MapMerge,
    MapPick,
    MapOmit,
    
    // List operations
    ListNew,
    ListGet,
    ListSet,
    ListPush,
    ListPop,
    ListLen,
    
    // String operations
    StringConcat,
    StringLen,
    StringSlice,
    StringToUpper,
    StringToLower,
    StringCapitalize,
    
    // Loop operators (from spec)
    LoopMap,       // $ operator
    LoopFilter,    // $? operator
    LoopFlatMap,   // $_ operator
    LoopUniqueBy,  // $~ operator
    LoopGroupBy,   // $> operator
    LoopEachToString,  // $" operator
    LoopEvery,     // $& operator
    LoopSome,      // $| operator
    LoopFind,      // $?| operator
    LoopSort,      // $% operator
    LoopCompact,   // $?! operator
    
    // Error handling
    Throw,         // !!! operator
    Catch,         // !!!? operator
    CatchEnd,
    
    // Async
    MakeAsync,     // /// operator
    Await,         // \\\ operator
    
    // Special
    ReverseArgs,   // . operator
    SilentExec,    // _ key in map
    Import,
    Export,
}

impl Opcode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Opcode::Nop),
            1 => Some(Opcode::Return),
            2 => Some(Opcode::Jump),
            3 => Some(Opcode::JumpIfTrue),
            4 => Some(Opcode::JumpIfFalse),
            5 => Some(Opcode::Call),
            6 => Some(Opcode::TailCall),
            10 => Some(Opcode::Push),
            11 => Some(Opcode::Pop),
            12 => Some(Opcode::Dup),
            20 => Some(Opcode::LoadConstant),
            21 => Some(Opcode::LoadRegister),
            22 => Some(Opcode::StoreRegister),
            23 => Some(Opcode::LoadLocal),
            24 => Some(Opcode::StoreLocal),
            30 => Some(Opcode::Add),
            31 => Some(Opcode::Sub),
            32 => Some(Opcode::Mul),
            33 => Some(Opcode::Div),
            34 => Some(Opcode::Mod),
            35 => Some(Opcode::Neg),
            40 => Some(Opcode::Eq),
            41 => Some(Opcode::Ne),
            42 => Some(Opcode::Lt),
            43 => Some(Opcode::Le),
            44 => Some(Opcode::Gt),
            45 => Some(Opcode::Ge),
            50 => Some(Opcode::Not),
            51 => Some(Opcode::And),
            52 => Some(Opcode::Or),
            53 => Some(Opcode::ToBoolean),
            60 => Some(Opcode::MapNew),
            61 => Some(Opcode::MapGet),
            62 => Some(Opcode::MapSet),
            63 => Some(Opcode::MapMerge),
            64 => Some(Opcode::MapPick),
            65 => Some(Opcode::MapOmit),
            70 => Some(Opcode::ListNew),
            71 => Some(Opcode::ListGet),
            72 => Some(Opcode::ListSet),
            73 => Some(Opcode::ListPush),
            74 => Some(Opcode::ListPop),
            75 => Some(Opcode::ListLen),
            80 => Some(Opcode::StringConcat),
            81 => Some(Opcode::StringLen),
            82 => Some(Opcode::StringSlice),
            83 => Some(Opcode::StringToUpper),
            84 => Some(Opcode::StringToLower),
            85 => Some(Opcode::StringCapitalize),
            90 => Some(Opcode::LoopMap),
            91 => Some(Opcode::LoopFilter),
            92 => Some(Opcode::LoopFlatMap),
            93 => Some(Opcode::LoopUniqueBy),
            94 => Some(Opcode::LoopGroupBy),
            95 => Some(Opcode::LoopEachToString),
            96 => Some(Opcode::LoopEvery),
            97 => Some(Opcode::LoopSome),
            98 => Some(Opcode::LoopFind),
            99 => Some(Opcode::LoopSort),
            100 => Some(Opcode::LoopCompact),
            110 => Some(Opcode::Throw),
            111 => Some(Opcode::Catch),
            112 => Some(Opcode::CatchEnd),
            120 => Some(Opcode::MakeAsync),
            121 => Some(Opcode::Await),
            130 => Some(Opcode::ReverseArgs),
            131 => Some(Opcode::SilentExec),
            140 => Some(Opcode::Import),
            141 => Some(Opcode::Export),
            _ => None,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Opcode::Nop => "NOP",
            Opcode::Return => "RET",
            Opcode::Jump => "JMP",
            Opcode::JumpIfTrue => "JMP_T",
            Opcode::JumpIfFalse => "JMP_F",
            Opcode::Call => "CALL",
            Opcode::TailCall => "TAIL_CALL",
            Opcode::Push => "PUSH",
            Opcode::Pop => "POP",
            Opcode::Dup => "DUP",
            Opcode::LoadConstant => "LOAD_CONST",
            Opcode::LoadRegister => "LOAD_REG",
            Opcode::StoreRegister => "STORE_REG",
            Opcode::LoadLocal => "LOAD_LOCAL",
            Opcode::StoreLocal => "STORE_LOCAL",
            Opcode::Add => "ADD",
            Opcode::Sub => "SUB",
            Opcode::Mul => "MUL",
            Opcode::Div => "DIV",
            Opcode::Mod => "MOD",
            Opcode::Neg => "NEG",
            Opcode::Eq => "EQ",
            Opcode::Ne => "NE",
            Opcode::Lt => "LT",
            Opcode::Le => "LE",
            Opcode::Gt => "GT",
            Opcode::Ge => "GE",
            Opcode::Not => "NOT",
            Opcode::And => "AND",
            Opcode::Or => "OR",
            Opcode::ToBoolean => "TO_BOOL",
            Opcode::MapNew => "MAP_NEW",
            Opcode::MapGet => "MAP_GET",
            Opcode::MapSet => "MAP_SET",
            Opcode::MapMerge => "MAP_MERGE",
            Opcode::MapPick => "MAP_PICK",
            Opcode::MapOmit => "MAP_OMIT",
            Opcode::ListNew => "LIST_NEW",
            Opcode::ListGet => "LIST_GET",
            Opcode::ListSet => "LIST_SET",
            Opcode::ListPush => "LIST_PUSH",
            Opcode::ListPop => "LIST_POP",
            Opcode::ListLen => "LIST_LEN",
            Opcode::StringConcat => "STR_CAT",
            Opcode::StringLen => "STR_LEN",
            Opcode::StringSlice => "STR_SLICE",
            Opcode::StringToUpper => "STR_UPPER",
            Opcode::StringToLower => "STR_LOWER",
            Opcode::StringCapitalize => "STR_CAP",
            Opcode::LoopMap => "LOOP_MAP",
            Opcode::LoopFilter => "LOOP_FILTER",
            Opcode::LoopFlatMap => "LOOP_FLATMAP",
            Opcode::LoopUniqueBy => "LOOP_UNIQUE",
            Opcode::LoopGroupBy => "LOOP_GROUP",
            Opcode::LoopEachToString => "LOOP_EACH_STR",
            Opcode::LoopEvery => "LOOP_EVERY",
            Opcode::LoopSome => "LOOP_SOME",
            Opcode::LoopFind => "LOOP_FIND",
            Opcode::LoopSort => "LOOP_SORT",
            Opcode::LoopCompact => "LOOP_COMPACT",
            Opcode::Throw => "THROW",
            Opcode::Catch => "CATCH",
            Opcode::CatchEnd => "CATCH_END",
            Opcode::MakeAsync => "MAKE_ASYNC",
            Opcode::Await => "AWAIT",
            Opcode::ReverseArgs => "REVERSE_ARGS",
            Opcode::SilentExec => "SILENT_EXEC",
            Opcode::Import => "IMPORT",
            Opcode::Export => "EXPORT",
        };
        write!(f, "{}", name)
    }
}

/// Value type tags for VM (7 types)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ValueType {
    Undefined = 0,
    Boolean = 1,
    Number = 2,
    String = 3,
    List = 4,
    Map = 5,
    Operator = 6,
}

/// Constant pool entry
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Undefined,
    Boolean(bool),
    Number(f64),
    String(String),
}

/// Bytecode chunk
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub code: Vec<Instruction>,
    pub constants: Vec<Constant>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.code.push(instruction);
    }

    pub fn add_constant(&mut self, constant: Constant) -> u8 {
        self.constants.push(constant);
        (self.constants.len() - 1) as u8
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 3: Write lr-bytecode/src/lib.rs**

```rust
pub mod instruction;

pub use instruction::*;
```

- [ ] **Step 4: Write instruction roundtrip test**

Create: `compiler/crates/lr-bytecode/tests/roundtrip.rs`

```rust
use lr_bytecode::{Instruction, Opcode};

#[test]
fn instruction_roundtrip() {
    let instr = Instruction::new(Opcode::Add as u8, 1, 2, 3);
    assert_eq!(instr.opcode(), Opcode::Add as u8);
    assert_eq!(instr.a(), 1);
    assert_eq!(instr.b(), 2);
    assert_eq!(instr.c(), 3);
}

#[test]
fn instruction_all_opcodes() {
    for opcode in 0..=255 {
        if let Some(op) = Opcode::from_u8(opcode) {
            let instr = Instruction::new(opcode, 10, 20, 30);
            assert_eq!(instr.opcode(), opcode);
            assert_eq!(Opcode::from_u8(instr.opcode()), Some(op));
        }
    }
}

#[test]
fn chunk_operations() {
    use lr_bytecode::{Chunk, Constant};
    
    let mut chunk = Chunk::new();
    let idx = chunk.add_constant(Constant::Number(42.0));
    
    assert_eq!(idx, 0);
    assert_eq!(chunk.constants.len(), 1);
    
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 5, idx, 0));
    assert_eq!(chunk.code.len(), 1);
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p lr-bytecode`
Expected: All tests pass

- [ ] **Step 6: Commit**

```bash
git add compiler/crates/lr-bytecode/
git commit -m "feat: add lr-bytecode crate with 32-bit instruction encoding"
```

---

### Task 4: Create lr-vm Crate (Register-based VM)

**Files:**
- Create: `compiler/crates/lr-vm/Cargo.toml`
- Create: `compiler/crates/lr-vm/src/lib.rs`
- Create: `compiler/crates/lr-vm/src/value.rs`
- Create: `compiler/crates/lr-vm/src/vm.rs`

- [ ] **Step 1: Write lr-vm/Cargo.toml**

```toml
[package]
name = "lr-vm"
version = "0.1.0"
edition = "2021"

[dependencies]
gc-arena = { workspace = true }
thiserror = { workspace = true }
lr-bytecode = { path = "../lr-bytecode" }

[dev-dependencies]
goldenfile = { workspace = true }
proptest = { workspace = true }
criterion = { workspace = true }
```

- [ ] **Step 2: Write lr-vm/src/value.rs**

```rust
//! Left-Right VM Value Types
//!
//! 7 types: Undefined, Boolean, Number, String, List, Map, Operator
//! Uses gc-arena for safe, incremental, exact, cycle-detecting GC
//! Reference: https://docs.rs/gc-arena/latest/gc_arena/

use std::fmt;
use gc_arena::{Collect, Gc, Mutation};
use lr_bytecode::ValueType;

/// VM Value (tagged with GC handles)
#[derive(Clone, Copy, Collect, Debug)]
#[gc(unsafe_drop)]
pub struct Value<'gc> {
    pub type_tag: ValueType,
    // TODO: Add value data with GC handles
}

impl<'gc> Value<'gc> {
    pub fn undefined(mc: &Mutation<'gc>) -> Self {
        Value {
            type_tag: ValueType::Undefined,
        }
    }

    pub fn boolean(mc: &Mutation<'gc>, value: bool) -> Self {
        Value {
            type_tag: ValueType::Boolean,
        }
    }

    pub fn number(mc: &Mutation<'gc>, value: f64) -> Self {
        Value {
            type_tag: ValueType::Number,
        }
    }

    pub fn is_truthy(self) -> bool {
        match self.type_tag {
            ValueType::Undefined => false,
            ValueType::Boolean => todo!(), // Extract value
            ValueType::Number => todo!(),  // Non-zero is truthy
            ValueType::String => true,     // Non-empty string is truthy
            ValueType::List => true,       // Non-empty list is truthy
            ValueType::Map => true,        // Non-empty map is truthy
            ValueType::Operator => true,
        }
    }
}

impl<'gc> fmt::Display for Value<'gc> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_tag {
            ValueType::Undefined => write!(f, "undefined"),
            ValueType::Boolean => write!(f, "true/false"), // TODO
            ValueType::Number => write!(f, "number"),      // TODO
            ValueType::String => write!(f, "string"),       // TODO
            ValueType::List => write!(f, "[...]"),          // TODO
            ValueType::Map => write!(f, "{{...}}"),         // TODO
            ValueType::Operator => write!(f, "<operator>"),
        }
    }
}

impl<'gc> PartialEq for Value<'gc> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Implement deep equality
        self.type_tag == other.type_tag
    }
}
```

- [ ] **Step 3: Write lr-vm/src/vm.rs (dispatch loop)**

```rust
//! Left-Right Register-Based VM
//!
//! Register-based dispatch loop (Lua-style 32-bit instructions)
//! Reference: https://luaf.dev/pages/virtualmachine.html

use std::cell::RefCell;
use gc_arena::{Arena, Collect, Gc, Rootable};
use lr_bytecode::{Chunk, Instruction, Opcode};
use crate::value::Value;

/// VM context with GC arena
#[derive(Collect)]
#[gc(no_drop)]
pub struct VMContext<'gc> {
    // TODO: Add VM state
}

/// Register frame
pub struct Frame<'gc> {
    registers: [Value<'gc>; 256],
    pc: usize,
}

impl<'gc> Frame<'gc> {
    pub fn new() -> Self {
        Frame {
            registers: [Value { type_tag: lr_bytecode::ValueType::Undefined }; 256],
            pc: 0,
        }
    }

    pub fn get(&self, reg: u8) -> Value<'gc> {
        self.registers[reg as usize]
    }

    pub fn set(&mut self, reg: u8, value: Value<'gc>) {
        self.registers[reg as usize] = value;
    }
}

impl<'gc> Default for Frame<'gc> {
    fn default() -> Self {
        Self::new()
    }
}

/// Main VM struct
pub struct VM<'gc> {
    arena: Arena<VMContext<'gc>>,
    frame: Frame<'gc>,
}

impl<'gc> VM<'gc> {
    pub fn new() -> Self {
        VM {
            arena: Arena::new(|mc| VMContext {}),
            frame: Frame::new(),
        }
    }

    /// Execute a chunk of bytecode
    pub fn execute(&mut self, chunk: &Chunk) -> Result<Value<'gc>, VMError> {
        loop {
            if self.frame.pc >= chunk.code.len() {
                return Ok(self.frame.get(0));
            }

            let instruction = chunk.code[self.frame.pc];
            self.frame.pc += 1;

            if let Err(e) = self.execute_instruction(instruction, chunk) {
                return Err(e);
            }
        }
    }

    /// Execute a single instruction
    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        chunk: &Chunk,
    ) -> Result<(), VMError> {
        let opcode = Opcode::from_u8(instruction.opcode())
            .ok_or(VMError::InvalidOpcode(instruction.opcode()))?;

        match opcode {
            Opcode::Nop => {},
            
            Opcode::Add => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                // TODO: Implement addition with type dispatch
                self.frame.set(instruction.a(), b);
            }
            
            Opcode::LoadConstant => {
                let const_idx = instruction.b() as usize;
                if const_idx >= chunk.constants.len() {
                    return Err(VMError::ConstantIndexOutOfBounds(const_idx));
                }
                let constant = &chunk.constants[const_idx];
                // TODO: Load constant into register
            }
            
            _ => return Err(VMError::UnimplementedOpcode(opcode)),
        }

        Ok(())
    }
}

impl<'gc> Default for VM<'gc> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VMError {
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u8),
    
    #[error("Unimplemented opcode: {0:?}")]
    UnimplementedOpcode(Opcode),
    
    #[error("Constant index out of bounds: {0}")]
    ConstantIndexOutOfBounds(usize),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
}
```

- [ ] **Step 4: Write lr-vm/src/lib.rs**

```rust
pub mod value;
pub mod vm;

pub use value::Value;
pub use vm::{VM, VMError, Frame, VMContext};
```

- [ ] **Step 5: Write VM dispatch test**

Create: `compiler/crates/lr-vm/tests/dispatch.rs`

```rust
use lr_bytecode::{Chunk, Instruction, Opcode, Constant};
use lr_vm::VM;

#[test]
fn vm_nop_instruction() {
    let mut chunk = Chunk::new();
    chunk.emit(Instruction::new(Opcode::Nop as u8, 0, 0, 0));
    
    let mut vm = VM::new();
    let result = vm.execute(&chunk);
    
    assert!(result.is_ok());
}

#[test]
fn vm_load_constant_number() {
    let mut chunk = Chunk::new();
    let const_idx = chunk.add_constant(Constant::Number(42.0));
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 1, const_idx, 0));
    
    let mut vm = VM::new();
    let result = vm.execute(&chunk);
    
    assert!(result.is_ok());
}
```

- [ ] **Step 6: Run tests**

Run: `cargo test -p lr-vm`
Expected: Tests pass (some may fail due to TODOs, that's expected)

- [ ] **Step 7: Commit**

```bash
git add compiler/crates/lr-vm/
git commit -m "feat: add lr-vm crate with register-based dispatch loop"
```

---

## Part 2: Complete Bytecode VM Implementation

### Task 5: Implement VM Value Types with GC

**Files:**
- Modify: `compiler/crates/lr-vm/src/value.rs`

- [ ] **Step 1: Implement complete Value type with GC**

```rust
//! Left-Right VM Value Types (complete implementation)
//!
//! 7 types: Undefined, Boolean, Number, String, List, Map, Operator
//! Uses gc-arena for safe, incremental, exact, cycle-detecting GC

use std::fmt;
use std::cmp::PartialEq;
use gc_arena::{Collect, Gc, Mutation, Rootable};
use lr_bytecode::ValueType;

/// VM Value (tagged with GC handles)
#[derive(Clone, Copy, Collect, Debug)]
#[gc(unsafe_drop)]
pub struct Value<'gc> {
    pub type_tag: ValueType,
    pub data: ValueData<'gc>,
}

#[derive(Clone, Copy, Collect, Debug)]
#[gc(unsafe_drop)]
pub enum ValueData<'gc> {
    Undefined,
    Boolean(bool),
    Number(f64),
    String(Gc<'gc, String>),
    List(Gc<'gc, Vec<Value<'gc>>>),
    Map(Gc<'gc, Vec<(Value<'gc>, Value<'gc>)>>),
    Operator(Gc<'gc, Box<dyn Fn(Value<'gc>, &Mutation<'gc>) -> Value<'gc> + 'gc>>),
}

impl<'gc> Value<'gc> {
    pub fn undefined(mc: &Mutation<'gc>) -> Self {
        Value {
            type_tag: ValueType::Undefined,
            data: ValueData::Undefined,
        }
    }

    pub fn boolean(mc: &Mutation<'gc>, value: bool) -> Self {
        Value {
            type_tag: ValueType::Boolean,
            data: ValueData::Boolean(value),
        }
    }

    pub fn number(mc: &Mutation<'gc>, value: f64) -> Self {
        Value {
            type_tag: ValueType::Number,
            data: ValueData::Number(value),
        }
    }

    pub fn string(mc: &Mutation<'gc>, value: String) -> Self {
        Value {
            type_tag: ValueType::String,
            data: ValueData::String(Gc::new(mc, value)),
        }
    }

    pub fn list(mc: &Mutation<'gc>, elements: Vec<Value<'gc>>) -> Self {
        Value {
            type_tag: ValueType::List,
            data: ValueData::List(Gc::new(mc, elements)),
        }
    }

    pub fn map(mc: &Mutation<'gc>, entries: Vec<(Value<'gc>, Value<'gc>)>) -> Self {
        Value {
            type_tag: ValueType::Map,
            data: ValueData::Map(Gc::new(mc, entries)),
        }
    }

    pub fn is_truthy(self) -> bool {
        match self.data {
            ValueData::Undefined => false,
            ValueData::Boolean(b) => b,
            ValueData::Number(n) => n != 0.0 && !n.is_nan(),
            ValueData::String(s) => !s.is_empty(),
            ValueData::List(l) => !l.is_empty(),
            ValueData::Map(m) => !m.is_empty(),
            ValueData::Operator(_) => true,
        }
    }

    pub fn as_number(self) -> Option<f64> {
        match self.data {
            ValueData::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_boolean(self) -> Option<bool> {
        match self.data {
            ValueData::Boolean(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_string(self) -> Option<Gc<'gc, String>> {
        match self.data {
            ValueData::String(s) => Some(s),
            _ => None,
        }
    }
}

impl<'gc> fmt::Display for Value<'gc> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ValueData::Undefined => write!(f, "undefined"),
            ValueData::Boolean(b) => write!(f, "{}", b),
            ValueData::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            ValueData::String(s) => write!(f, "{}", s),
            ValueData::List(l) => {
                write!(f, "[")?;
                for (i, elem) in l.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            ValueData::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            ValueData::Operator(_) => write!(f, "<operator>"),
        }
    }
}

impl<'gc> PartialEq for Value<'gc> {
    fn eq(&self, other: &Self) -> bool {
        if self.type_tag != other.type_tag {
            return false;
        }
        
        match (&self.data, &other.data) {
            (ValueData::Undefined, ValueData::Undefined) => true,
            (ValueData::Boolean(a), ValueData::Boolean(b)) => a == b,
            (ValueData::Number(a), ValueData::Number(b)) => a == b,
            (ValueData::String(a), ValueData::String(b)) => a == b,
            (ValueData::List(a), ValueData::List(b)) => {
                // For lists, compare by pointer identity
                std::ptr::eq(&**a, &**b)
            }
            (ValueData::Map(a), ValueData::Map(b)) => {
                // For maps, compare by pointer identity
                std::ptr::eq(&**a, &**b)
            }
            (ValueData::Operator(a), ValueData::Operator(b)) => {
                // For operators, compare by pointer identity
                std::ptr::eq(&**a, &**b)
            }
            _ => false,
        }
    }
}
```

- [ ] **Step 2: Write VM value tests**

Create: `compiler/crates/lr-vm/tests/values.rs`

```rust
use gc_arena::Arena;
use lr_bytecode::ValueType;
use lr_vm::Value;

#[test]
fn value_undefined() {
    // Note: Rootable!() macro syntax may need adjustment based on gc-arena 0.5.3 API. Verify during implementation.
    let arena = Arena::<Rootable!()>::new(|mc| ());
    arena.mutate(|mc, root| {
        let val = Value::undefined(mc);
        assert_eq!(val.type_tag, ValueType::Undefined);
        assert!(!val.is_truthy());
    });
}

#[test]
fn value_boolean() {
    let arena = Arena::<Rootable!()>::new(|mc| ());
    arena.mutate(|mc, root| {
        let true_val = Value::boolean(mc, true);
        let false_val = Value::boolean(mc, false);
        
        assert_eq!(true_val.type_tag, ValueType::Boolean);
        assert!(true_val.is_truthy());
        assert!(!false_val.is_truthy());
        assert_eq!(true_val.as_boolean(), Some(true));
    });
}

#[test]
fn value_number() {
    let arena = Arena::<Rootable!()>::new(|mc| ());
    arena.mutate(|mc, root| {
        let zero = Value::number(mc, 0.0);
        let five = Value::number(mc, 5.0);
        let pi = Value::number(mc, 3.14);
        
        assert!(!zero.is_truthy());
        assert!(five.is_truthy());
        assert!(pi.is_truthy());
        
        assert_eq!(five.as_number(), Some(5.0));
        assert_eq!(pi.as_number(), Some(3.14));
    });
}

#[test]
fn value_string() {
    let arena = Arena::<Rootable!()>::new(|mc| ());
    arena.mutate(|mc, root| {
        let empty = Value::string(mc, "".to_string());
        let hello = Value::string(mc, "hello".to_string());
        
        assert!(!empty.is_truthy());
        assert!(hello.is_truthy());
        
        assert_eq!(format!("{}", hello), "hello");
    });
}

#[test]
fn value_list() {
    let arena = Arena::<Rootable!()>::new(|mc| ());
    arena.mutate(|mc, root| {
        let empty = Value::list(mc, vec![]);
        let numbers = Value::list(mc, vec![
            Value::number(mc, 1.0),
            Value::number(mc, 2.0),
        ]);
        
        assert!(!empty.is_truthy());
        assert!(numbers.is_truthy());
        
        assert_eq!(format!("{}", numbers), "[1, 2]");
    });
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p lr-vm values`
Expected: All value tests pass

- [ ] **Step 4: Commit**

```bash
git add compiler/crates/lr-vm/
git commit -m "feat: implement complete VM value types with GC"
```

---

### Task 6: Implement Complete VM Instruction Dispatch

**Files:**
- Modify: `compiler/crates/lr-vm/src/vm.rs`

- [ ] **Step 1: Implement full instruction dispatch**

```rust
//! Complete VM instruction dispatch
//!
//! Implements all opcodes with type-dependent operator behavior

use std::cell::RefCell;
use gc_arena::{Arena, Collect, Gc, Rootable, Mutation};
use lr_bytecode::{Chunk, Instruction, Opcode, Constant};
use crate::value::Value;
use crate::vm::{VMError, Frame, VMContext};

impl<'gc> VM<'gc> {
    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        chunk: &Chunk,
    ) -> Result<(), VMError> {
        let opcode = Opcode::from_u8(instruction.opcode())
            .ok_or(VMError::InvalidOpcode(instruction.opcode()))?;

        match opcode {
            // Control flow
            Opcode::Nop => {},
            
            Opcode::Return => {
                return Err(VMError::Return);
            }
            
            Opcode::Jump => {
                let offset = instruction.a() as i8 as i32;
                self.frame.pc = ((self.frame.pc as i32) + offset) as usize;
            }
            
            Opcode::JumpIfTrue => {
                let cond = self.frame.get(instruction.a());
                if cond.is_truthy() {
                    let offset = instruction.b() as i8 as i32;
                    self.frame.pc = ((self.frame.pc as i32) + offset) as usize;
                }
            }
            
            Opcode::JumpIfFalse => {
                let cond = self.frame.get(instruction.a());
                if !cond.is_truthy() {
                    let offset = instruction.b() as i8 as i32;
                    self.frame.pc = ((self.frame.pc as i32) + offset) as usize;
                }
            }
            
            Opcode::Call => {
                // TODO: Implement function calls
            }
            
            Opcode::TailCall => {
                // TODO: Implement tail calls
            }
            
            // Stack operations (simplified for now)
            Opcode::Push => {
                let val = self.frame.get(instruction.a());
                self.frame.set(instruction.a(), val);
            }
            
            Opcode::Pop => {
                // TODO: Implement stack
            }
            
            Opcode::Dup => {
                let val = self.frame.get(instruction.a());
                self.frame.set(instruction.b(), val);
            }
            
            // Load/Store
            Opcode::LoadConstant => {
                let const_idx = instruction.b() as usize;
                if const_idx >= chunk.constants.len() {
                    return Err(VMError::ConstantIndexOutOfBounds(const_idx));
                }
                
                self.arena.mutate(|mc, root| {
                    let constant = &chunk.constants[const_idx];
                    let val = match constant {
                        Constant::Undefined => Value::undefined(mc),
                        Constant::Boolean(b) => Value::boolean(mc, *b),
                        Constant::Number(n) => Value::number(mc, *n),
                        Constant::String(s) => Value::string(mc, s.clone()),
                    };
                    self.frame.set(instruction.a(), val);
                });
            }
            
            Opcode::LoadRegister => {
                let val = self.frame.get(instruction.b());
                self.frame.set(instruction.a(), val);
            }
            
            Opcode::StoreRegister => {
                let val = self.frame.get(instruction.a());
                self.frame.set(instruction.b(), val);
            }
            
            // Arithmetic (type-dependent)
            Opcode::Add => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    let result = match (b.data, c.data) {
                        (ValueData::Number(x), ValueData::Number(y)) => {
                            Value::number(mc, x + y)
                        }
                        (ValueData::String(s), ValueData::String(t)) => {
                            Value::string(mc, format!("{}{}", s, t))
                        }
                        _ => {
                            // TODO: Map merge, list concat
                            return Err(VMError::Runtime("Invalid types for addition".to_string()));
                        }
                    };
                    self.frame.set(instruction.a(), result);
                })?;
            }
            
            Opcode::Sub => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        let result = Value::number(mc, x - y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for subtraction".to_string()));
                    }
                })?;
            }
            
            Opcode::Mul => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        let result = Value::number(mc, x * y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for multiplication".to_string()));
                    }
                })?;
            }
            
            Opcode::Div => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        if *y == 0.0 {
                            return Err(VMError::Runtime("Division by zero".to_string()));
                        }
                        let result = Value::number(mc, x / y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for division".to_string()));
                    }
                })?;
            }
            
            Opcode::Mod => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        if *y == 0.0 {
                            return Err(VMError::Runtime("Modulo by zero".to_string()));
                        }
                        let result = Value::number(mc, x % y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for modulo".to_string()));
                    }
                })?;
            }
            
            Opcode::Neg => {
                let b = self.frame.get(instruction.b());
                
                self.arena.mutate(|mc, root| {
                    if let Some(x) = b.as_number() {
                        let result = Value::number(mc, -x);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid type for negation".to_string()));
                    }
                })?;
            }
            
            // Comparison
            Opcode::Eq => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    let result = Value::boolean(mc, b == c);
                    self.frame.set(instruction.a(), result);
                });
            }
            
            Opcode::Ne => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    let result = Value::boolean(mc, b != c);
                    self.frame.set(instruction.a(), result);
                });
            }
            
            Opcode::Lt => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        let result = Value::boolean(mc, x < y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for comparison".to_string()));
                    }
                })?;
            }
            
            Opcode::Le => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        let result = Value::boolean(mc, x <= y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for comparison".to_string()));
                    }
                })?;
            }
            
            Opcode::Gt => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        let result = Value::boolean(mc, x > y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for comparison".to_string()));
                    }
                })?;
            }
            
            Opcode::Ge => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    if let (Some(x), Some(y)) = (b.as_number(), c.as_number()) {
                        let result = Value::boolean(mc, x >= y);
                        self.frame.set(instruction.a(), result);
                    } else {
                        return Err(VMError::Runtime("Invalid types for comparison".to_string()));
                    }
                })?;
            }
            
            // Boolean
            Opcode::Not => {
                let b = self.frame.get(instruction.b());
                
                self.arena.mutate(|mc, root| {
                    let result = Value::boolean(mc, !b.is_truthy());
                    self.frame.set(instruction.a(), result);
                });
            }
            
            Opcode::And => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    let result = Value::boolean(mc, b.is_truthy() && c.is_truthy());
                    self.frame.set(instruction.a(), result);
                });
            }
            
            Opcode::Or => {
                let b = self.frame.get(instruction.b());
                let c = self.frame.get(instruction.c());
                
                self.arena.mutate(|mc, root| {
                    let result = Value::boolean(mc, b.is_truthy() || c.is_truthy());
                    self.frame.set(instruction.a(), result);
                });
            }
            
            Opcode::ToBoolean => {
                let b = self.frame.get(instruction.b());
                
                self.arena.mutate(|mc, root| {
                    let result = Value::boolean(mc, b.is_truthy());
                    self.frame.set(instruction.a(), result);
                });
            }
            
            // TODO: Implement remaining opcodes (Map, List, String, Loop, Error, Async)
            
            _ => return Err(VMError::UnimplementedOpcode(opcode)),
        }

        Ok(())
    }
}
```

- [ ] **Step 2: Write instruction dispatch tests**

Create: `compiler/crates/lr-vm/tests/dispatch_complete.rs`

```rust
use lr_bytecode::{Chunk, Instruction, Opcode, Constant};
use lr_vm::VM;

#[test]
fn vm_arithmetic() {
    let mut chunk = Chunk::new();
    let five = chunk.add_constant(Constant::Number(5.0));
    let three = chunk.add_constant(Constant::Number(3.0));
    
    // Load constants
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 1, five, 0));
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 2, three, 0));
    
    // Add: 5 + 3 = 8
    chunk.emit(Instruction::new(Opcode::Add as u8, 3, 1, 2));
    
    // Return value in r3
    chunk.emit(Instruction::new(Opcode::LoadRegister as u8, 0, 3, 0));
    chunk.emit(Instruction::new(Opcode::Return as u8, 0, 0, 0));
    
    let mut vm = VM::new();
    let result = vm.execute(&chunk);
    
    assert!(result.is_ok());
}

#[test]
fn vm_comparison() {
    let mut chunk = Chunk::new();
    let five = chunk.add_constant(Constant::Number(5.0));
    let three = chunk.add_constant(Constant::Number(3.0));
    
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 1, five, 0));
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 2, three, 0));
    
    // 5 > 3 = true
    chunk.emit(Instruction::new(Opcode::Gt as u8, 3, 1, 2));
    
    chunk.emit(Instruction::new(Opcode::LoadRegister as u8, 0, 3, 0));
    chunk.emit(Instruction::new(Opcode::Return as u8, 0, 0, 0));
    
    let mut vm = VM::new();
    let result = vm.execute(&chunk);
    
    assert!(result.is_ok());
}

#[test]
fn vm_boolean() {
    let mut chunk = Chunk::new();
    let true_val = chunk.add_constant(Constant::Boolean(true));
    let false_val = chunk.add_constant(Constant::Boolean(false));
    
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 1, true_val, 0));
    chunk.emit(Instruction::new(Opcode::LoadConstant as u8, 2, false_val, 0));
    
    // true AND false = false
    chunk.emit(Instruction::new(Opcode::And as u8, 3, 1, 2));
    
    chunk.emit(Instruction::new(Opcode::LoadRegister as u8, 0, 3, 0));
    chunk.emit(Instruction::new(Opcode::Return as u8, 0, 0, 0));
    
    let mut vm = VM::new();
    let result = vm.execute(&chunk);
    
    assert!(result.is_ok());
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p lr-vm dispatch_complete`
Expected: Tests pass

- [ ] **Step 4: Commit**

```bash
git add compiler/crates/lr-vm/
git commit -m "feat: implement complete VM instruction dispatch (arithmetic, comparison, boolean)"
```

---

## Live Testing Criteria (Comprehensive)

For EACH of the 20 test flows, create a test file with:
1. Input source (`.lr` code)
2. Expected bytecode (simplified representation)
3. Expected runtime result

**Test 1: Basic Arithmetic**
- Input: `5 + 3`
- Expected result: 8
- Key: Zero precedence, left-to-right evaluation

**Test 2: Zero Precedence**
- Input: `5 + 3 * 2`
- Expected result: 16 (not 11)
- Key: `((5 + 3) * 2)`, NOT `(5 + (3 * 2))`

**Test 3: Curried Application**
- Input: `add 1 2` (where add is `{ _< + _> }`)
- Expected result: 3
- Key: Map as function with left/right args

**Test 4: Map as Function**
- Input: `{ x: x + 1 }` applied to 5
- Expected result: 6
- Key: Map with `_<` reference = unexecuted operator

**Test 5: Map as Conditional**
- Input: `{ _<: `yes`, `no` }` with truthy left arg
- Expected result: `yes`
- Key: Expression key `_<` with `:` = early return

**Test 6: String Interpolation**
- Input: `` `{name} is {age}` ``
- Expected result: correctly interpolated string
- Key: `{expr}` inside backtick string

**Test 7: List Operations**
- Input: `[1, 2, 3] #`
- Expected result: 3
- Key: `#` operator gets size

**Test 8: Map Operations**
- Input: `{ a: 1, b: 2 } @ `a``
- Expected result: 1
- Key: `@` operator gets property

**Test 9: Error Handling**
- Input: `value !!!` throws, `!!!?` catches
- Expected result: caught error
- Key: `!!!` throw, `!!!?` catch (identifiers)

**Test 10: Async**
- Input: `///` marks async, `\\\` awaits
- Expected result: awaited async result
- Key: Async primitives as identifiers

**Test 11: Import**
- Input: `imports@[`lodash`]` resolves
- Expected result: imported module
- Key: `imports` runtime variable, `@` get

**Test 12: Export**
- Input: `}@&[export1]` works
- Expected result: exported values
- Key: Export pattern

**Test 13: Nested Maps/Lists**
- Input: `{ a: [1, 2], b: { c: 3 } }`
- Expected result: correctly nested structure
- Key: Universal map nesting

**Test 14: Reverse-Args**
- Input: `value @.map.` → correct
- Expected result: swapped left/right slots
- Key: `.` operator reverses arguments

**Test 15: Spread/Merge**
- Input: `mapA +: mapB` → merged
- Expected result: merged map
- Key: `+:` spread/merge

**Test 16: Silent Execution**
- Input: `_: sideEffect` runs but returns undefined
- Expected result: undefined
- Key: `_` key = no-output execution

**Test 17: Loop Operators**
- Input: `$ map` over list
- Expected result: mapped list
- Key: Loop operators as identifiers

**Test 18: Filter**
- Input: `$?` filter over list
- Expected result: filtered list
- Key: Filter operator

**Test 19: Reduce Pattern**
- Input: Chained loop operators
- Expected result: reduced value
- Key: Composition of loop operators

**Test 20: Chained Operations**
- Input: `data $ filter $ map $ reduce`
- Expected result: fully transformed data
- Key: Complex operator chains

---

## Implementation Order

### Phase 1: Instruction Set + Value Types (With Unit Tests) ✅
- [x] Instruction encoding (32-bit fixed-width)
- [x] Opcode enumeration (all categories)
- [x] Value type tags (7 types)
- [x] GC integration with gc-arena
- [x] Unit tests for encoding/decoding roundtrip

### Phase 2: VM Dispatch Loop + Basic Operations ✅
- [x] Register frame structure
- [x] Dispatch loop
- [x] Value representation (tagged union)
- [x] Basic opcodes: Nop, Return, LoadConstant, Load/Store Register
- [x] Arithmetic: Add, Sub, Mul, Div, Mod, Neg
- [x] Comparison: Eq, Ne, Lt, Le, Gt, Ge
- [x] Boolean: Not, And, Or, ToBoolean
- [ ] Unit tests for each opcode category

### Phase 3: Compiler Core (AST → Bytecode)
- [ ] Create lr-compiler crate
- [ ] AST to IR lowering
- [ ] IR to bytecode lowering
- [ ] Map compilation (function/conditional/control-flow)
- [ ] Operator compilation (curried chains)
- [ ] String interpolation compilation
- [ ] Unit tests for each compilation phase

### Phase 4: Map/List/String Compilation
- [ ] Map operations (get, set, merge, pick, omit)
- [ ] List operations (get, set, push, pop, len)
- [ ] String operations (concat, len, slice, case conversion)
- [ ] Nested structure compilation
- [ ] Golden tests for map/list/string operations

### Phase 5: Runtime Library (All Operators)
- [ ] Standard operators (`+`, `-`, `*`, `/`, `@`, `@&`, `@-`, `@|`, `.`, `=`, `|`, `&`, `!`, `#`, `?`)
- [ ] Loop operators (`$`, `$@`, `$?`, `$_`, `$~`, `$>`, `$"`, `$&`, `$|`, `$?|`, `$%`, `$?!`)
- [ ] String operators (`"`, `"_`, `"^`, `"^_`, `"~`, `<>`, `><`)
- [ ] Boolean operators (`?"`, `?#`, `?><`)
- [ ] Import/export runtime support
- [ ] Tests for each operator

### Phase 6: GC Integration
- [ ] Full gc-arena integration
- [ ] Root set tracking
- [ ] Write barriers (if needed)
- [ ] Cycle detection verification
- [ ] Memory leak tests

### Phase 7: Live System Test Suite
- [ ] All 20 test flows with golden tests
- [ ] End-to-end compilation + execution tests
- [ ] Performance benchmarks
- [ ] Error case coverage

### Phase 8: Performance Optimizations (DEFERRED)

**NOTE: This phase is intentionally DEFERRED. Implement only after baseline VM passes all tests (Phase 1-7 complete).**

Do not implement inline caching, shapes, or hidden classes in the initial implementation. These are optimization techniques that should be added only after verifying correct functionality.

#### 8.1 Inline Caching (4-20% speedup)
- [ ] Monomorphic inline caching (single type fast path)
  - Pattern: `BTreeMap<usize, usize>` (bytecode offset → cache slot)
  - Per-instruction cache, keyed by shape/type
  - Reference: som-rs implementation
- [ ] Polymorphic inline caching (2-4 types)
  - Small vector of type-specialized paths
  - Fallback to megamorphic when cache overflows
- [ ] Megamorphic fallback
  - Dictionary lookup for rare type combinations
- [ ] Inline cache tests and benchmarks
- References:
  - som-rs pattern: https://github.com/OctaveLarose/som-rs/commit/6a258977d25a051589c73b0663aa0274334e45b1
  - V8 IC article: https://debuglab.net/2026/05/10/inside-v8-deoptimization-how-inline-caches-distort-javascript/

#### 8.2 Shapes / Hidden Classes for Maps
- [ ] Track property addition order
  - Each map shape has unique ID based on property sequence
  - Monomorphic ICs keyed by shape ID (not individual properties)
- [ ] Transition chains
  - Root → prototype → insert transitions
  - Fast path: follow transition chain to find property offset
- [ ] Shape-based map optimization tests
- References:
  - Boa shapes implementation: https://boajs.dev/docs/intro
  - V8 hidden classes: https://debuglab.net/2026/05/10/inside-v8-deoptimization-how-inline-caches-distort-javascript/

---

## Critical Implementation Notes

### All Operators Are Identifiers
- `+`, `@`, `!!!`, `!!!?`, `///`, `\\\`, `$@` are ALL `Identifier` tokens
- No separate `Operator` token type in AST
- Runtime dispatch based on VALUE type, not token type

### Curried Evaluation
- Every operator takes left arg, may take right arg
- `5 + 3` = `((5+)3)` → 8
- Zero precedence: strict left-to-right
- **Currying note**: Use push-enter model — evaluate arguments first, then invoke. Currying creates small closures with environment chaining. Compiler detects when function is fully applied vs partially applied. Register-based possible but requires careful closure representation.

### Async Operator Approach
- `!` suffix makes an operator async (e.g., `///`, `\\\`)
- At VM level: `!` operators compile to async closures
- `AWAIT` opcode (opcode 121) pauses async execution
- Use Rust's native async/await (zero-cost state machines via generators)
- Tokio runtime for async task scheduling
- NOT green threads, NOT custom coroutine implementation
- Async values: boxed `Future` in heap, GC-managed

### Maps Are Universal
- Maps serve as: functions, conditionals, control flow, data storage
- Map with `_<`/`_>` inside = unexecuted operator
- Map with expression key + `:` = early return

### 7 Types Only
- Operator, Map, List, String, Boolean, Number, Undefined
- NO `Null` type — use `Undefined` instead

### gc-arena Integration
- Safe, incremental, exact, cycle-detecting GC
- Zero-overhead `Gc` pointers
- Used by Ruffle and piccolo
- Reference: https://docs.rs/gc-arena/latest/gc_arena/

### Golden Testing
- Use `goldenfile` v1.11.0 for golden tests
- Alternative: okane-golden (actively maintained fork)
- Reference: https://docs.rs/goldenfile/latest/goldenfile/

---

## Verification Checklist

Before claiming implementation complete, verify:

**Translation Gap Notes:**
The following language features found in translations need compiler support:
- **Constructor call**: `Error[expr]` — Application with Identifier("Error") and ListLiteral args
- **Default parameter**: `_<@2 | 10` — `|` is Identifier, compiler handles as binary operator
- **Bracket access**: `@[`path`]` — Application chain

- [ ] All unit tests pass
- [ ] All 20 live system tests pass
- [ ] No LSP diagnostics errors
- [ ] Build succeeds (`cargo build --release`)
- [ ] Golden tests are up-to-date
- [ ] No memory leaks (verify with Valgrind if possible)
- [ ] All operators from spec are implemented
 - [ ] Async/await works correctly
 - [ ] Error handling works correctly
 - [ ] Import/export works correctly
 - [ ] GC handles cycles correctly
 - [ ] Baseline performance acceptable (target: <1ms for simple expressions)

**Note: Inline caching and shape optimization are Phase 8 (deferred), not required for baseline implementation.**

---

## Success Metrics

Implementation is successful when:

1. **All 20 test flows pass** - Every test case from the live testing criteria produces the expected result
2. **Zero LSP errors** - Clean diagnostics on all modified files
3. **Build passes** - `cargo build --release` succeeds
4. **Performance** - VM executes code within acceptable latency (target: <1ms for simple expressions)
5. **Memory safety** - No memory leaks, GC handles cycles
6. **Correctness** - Matches spec for all language constructs

---

## References

1. Lua VM (32-bit instructions): https://luaf.dev/pages/virtualmachine.html
2. Cranelift SSA pattern: https://docs.rs/cranelift-frontend/latest/cranelift_frontend/
3. Boa crate separation: https://boajs.dev/docs/intro
4. gc-arena: https://docs.rs/gc-arena/latest/gc_arena/
5. Inline caching: https://github.com/OctaveLarose/som-rs/commit/6a258977d25a051589c73b0663aa0274334e45b1
6. Golden testing: https://docs.rs/goldenfile/latest/goldenfile/
7. rilua: https://crates.io/crates/rilua/v0.1.21
8. Hidden classes: https://debuglab.net/2026/05/10/inside-v8-deoptimization-how-inline-caches-distort-javascript/
9. Cranelift e-graph: https://github.com/cfallin/rfcs/blob/main/accepted/cranelift-egraph.md
10. AST Specification: `docs/specs/ast-specification.md`
11. Language Specification: `docs/specs/left-right-language-specification.md`
12. IR and Optimization: `docs/reports/how-to-build-a-compiled-language/04-intermediate-representations-and-optimization.md`
13. Code Generation: `docs/reports/how-to-build-a-compiled-language/05-code-generation.md`
14. Runtime Systems: `docs/reports/how-to-build-a-compiled-language/06-runtime-systems-and-memory-management.md`

---

## Next Steps After Core VM

After completing Phase 7 (Live System Test Suite), proceed with:

1. **Optimization Passes**: Constant folding, dead code elimination, common subexpression elimination
2. **Phase 8: Performance Optimizations**: Inline caching, shapes/hidden classes (only after baseline verified)
3. **JIT Compilation**: Optional, for performance-critical code paths
4. **Debugging Support**: Add source maps, breakpoints, step-through debugging
5. **Profile-Guided Optimization**: Collect runtime feedback, optimize hot paths

---

**END OF IMPLEMENTATION PLAN**