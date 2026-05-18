use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0,
    Return = 1,
    Jump = 2,
    JumpIfTrue = 3,
    JumpIfFalse = 4,
    Call = 5,
    TailCall = 6,

    Push = 10,
    Pop = 11,
    Dup = 12,

    LoadConstant = 20,
    LoadRegister = 21,
    StoreRegister = 22,
    LoadLocal = 23,
    StoreLocal = 24,

    Add = 30,
    Sub = 31,
    Mul = 32,
    Div = 33,
    Mod = 34,
    Neg = 35,

    Eq = 40,
    Ne = 41,
    Lt = 42,
    Le = 43,
    Gt = 44,
    Ge = 45,

    Not = 50,
    And = 51,
    Or = 52,
    ToBoolean = 53,

    MapNew = 60,
    MapGet = 61,
    MapSet = 62,
    MapMerge = 63,
    MapPick = 64,
    MapOmit = 65,

    ListNew = 70,
    ListGet = 71,
    ListSet = 72,
    ListPush = 73,
    ListPop = 74,
    ListLen = 75,

    StringConcat = 80,
    StringLen = 81,
    StringSlice = 82,
    StringToUpper = 83,
    StringToLower = 84,
    StringCapitalize = 85,

    LoopMap = 90,
    LoopFilter = 91,
    LoopFlatMap = 92,
    LoopUniqueBy = 93,
    LoopGroupBy = 94,
    LoopEachToString = 95,
    LoopEvery = 96,
    LoopSome = 97,
    LoopFind = 98,
    LoopSort = 99,
    LoopCompact = 100,

    Throw = 110,
    Catch = 111,
    CatchEnd = 112,

    MakeAsync = 120,
    Await = 121,

    ReverseArgs = 130,
    SilentExec = 131,
    Import = 140,
    Export = 141,
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

impl Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Nop => write!(f, "NOP"),
            Opcode::Return => write!(f, "RET"),
            Opcode::Jump => write!(f, "JMP"),
            Opcode::JumpIfTrue => write!(f, "JMP_T"),
            Opcode::JumpIfFalse => write!(f, "JMP_F"),
            Opcode::Call => write!(f, "CALL"),
            Opcode::TailCall => write!(f, "TAIL_CALL"),

            Opcode::Push => write!(f, "PUSH"),
            Opcode::Pop => write!(f, "POP"),
            Opcode::Dup => write!(f, "DUP"),

            Opcode::LoadConstant => write!(f, "LOAD_CONST"),
            Opcode::LoadRegister => write!(f, "LOAD_REG"),
            Opcode::StoreRegister => write!(f, "STORE_REG"),
            Opcode::LoadLocal => write!(f, "LOAD_LOCAL"),
            Opcode::StoreLocal => write!(f, "STORE_LOCAL"),

            Opcode::Add => write!(f, "ADD"),
            Opcode::Sub => write!(f, "SUB"),
            Opcode::Mul => write!(f, "MUL"),
            Opcode::Div => write!(f, "DIV"),
            Opcode::Mod => write!(f, "MOD"),
            Opcode::Neg => write!(f, "NEG"),

            Opcode::Eq => write!(f, "EQ"),
            Opcode::Ne => write!(f, "NE"),
            Opcode::Lt => write!(f, "LT"),
            Opcode::Le => write!(f, "LE"),
            Opcode::Gt => write!(f, "GT"),
            Opcode::Ge => write!(f, "GE"),

            Opcode::Not => write!(f, "NOT"),
            Opcode::And => write!(f, "AND"),
            Opcode::Or => write!(f, "OR"),
            Opcode::ToBoolean => write!(f, "TO_BOOL"),

            Opcode::MapNew => write!(f, "MAP_NEW"),
            Opcode::MapGet => write!(f, "MAP_GET"),
            Opcode::MapSet => write!(f, "MAP_SET"),
            Opcode::MapMerge => write!(f, "MAP_MERGE"),
            Opcode::MapPick => write!(f, "MAP_PICK"),
            Opcode::MapOmit => write!(f, "MAP_OMIT"),

            Opcode::ListNew => write!(f, "LIST_NEW"),
            Opcode::ListGet => write!(f, "LIST_GET"),
            Opcode::ListSet => write!(f, "LIST_SET"),
            Opcode::ListPush => write!(f, "LIST_PUSH"),
            Opcode::ListPop => write!(f, "LIST_POP"),
            Opcode::ListLen => write!(f, "LIST_LEN"),

            Opcode::StringConcat => write!(f, "STR_CAT"),
            Opcode::StringLen => write!(f, "STR_LEN"),
            Opcode::StringSlice => write!(f, "STR_SLICE"),
            Opcode::StringToUpper => write!(f, "STR_UPPER"),
            Opcode::StringToLower => write!(f, "STR_LOWER"),
            Opcode::StringCapitalize => write!(f, "STR_CAP"),

            Opcode::LoopMap => write!(f, "LOOP_MAP"),
            Opcode::LoopFilter => write!(f, "LOOP_FILTER"),
            Opcode::LoopFlatMap => write!(f, "LOOP_FLATMAP"),
            Opcode::LoopUniqueBy => write!(f, "LOOP_UNIQUE"),
            Opcode::LoopGroupBy => write!(f, "LOOP_GROUP"),
            Opcode::LoopEachToString => write!(f, "LOOP_EACH_STR"),
            Opcode::LoopEvery => write!(f, "LOOP_EVERY"),
            Opcode::LoopSome => write!(f, "LOOP_SOME"),
            Opcode::LoopFind => write!(f, "LOOP_FIND"),
            Opcode::LoopSort => write!(f, "LOOP_SORT"),
            Opcode::LoopCompact => write!(f, "LOOP_COMPACT"),

            Opcode::Throw => write!(f, "THROW"),
            Opcode::Catch => write!(f, "CATCH"),
            Opcode::CatchEnd => write!(f, "CATCH_END"),

            Opcode::MakeAsync => write!(f, "MAKE_ASYNC"),
            Opcode::Await => write!(f, "AWAIT"),

            Opcode::ReverseArgs => write!(f, "REVERSE_ARGS"),
            Opcode::SilentExec => write!(f, "SILENT_EXEC"),
            Opcode::Import => write!(f, "IMPORT"),
            Opcode::Export => write!(f, "EXPORT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction(u32);

impl Instruction {
    pub fn new(opcode: Opcode, a: u8, b: u8, c: u8) -> Self {
        let opcode = opcode as u32;
        let a = a as u32;
        let b = b as u32;
        let c = c as u32;

        Instruction(opcode << 24 | a << 16 | b << 8 | c)
    }

    pub fn opcode(&self) -> Opcode {
        let opcode_byte = (self.0 >> 24) as u8;
        Opcode::from_u8(opcode_byte).expect("Invalid opcode in instruction")
    }

    pub fn a(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn b(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn c(&self) -> u8 {
        self.0 as u8
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode(), self.a(), self.b(), self.c())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_roundtrip() {
        let opcodes = [
            Opcode::Add,
            Opcode::Call,
            Opcode::JumpIfTrue,
            Opcode::LoadConstant,
            Opcode::ListPush,
        ];

        for opcode in opcodes {
            let a = 42u8;
            let b = 100u8;
            let c = 255u8;

            let inst = Instruction::new(opcode, a, b, c);
            assert_eq!(inst.opcode(), opcode);
            assert_eq!(inst.a(), a);
            assert_eq!(inst.b(), b);
            assert_eq!(inst.c(), c);
        }
    }

    #[test]
    fn instruction_all_opcodes() {
        let all_opcodes = [
            Opcode::Nop,
            Opcode::Return,
            Opcode::Jump,
            Opcode::JumpIfTrue,
            Opcode::JumpIfFalse,
            Opcode::Call,
            Opcode::TailCall,
            Opcode::Push,
            Opcode::Pop,
            Opcode::Dup,
            Opcode::LoadConstant,
            Opcode::LoadRegister,
            Opcode::StoreRegister,
            Opcode::LoadLocal,
            Opcode::StoreLocal,
            Opcode::Add,
            Opcode::Sub,
            Opcode::Mul,
            Opcode::Div,
            Opcode::Mod,
            Opcode::Neg,
            Opcode::Eq,
            Opcode::Ne,
            Opcode::Lt,
            Opcode::Le,
            Opcode::Gt,
            Opcode::Ge,
            Opcode::Not,
            Opcode::And,
            Opcode::Or,
            Opcode::ToBoolean,
            Opcode::MapNew,
            Opcode::MapGet,
            Opcode::MapSet,
            Opcode::MapMerge,
            Opcode::MapPick,
            Opcode::MapOmit,
            Opcode::ListNew,
            Opcode::ListGet,
            Opcode::ListSet,
            Opcode::ListPush,
            Opcode::ListPop,
            Opcode::ListLen,
            Opcode::StringConcat,
            Opcode::StringLen,
            Opcode::StringSlice,
            Opcode::StringToUpper,
            Opcode::StringToLower,
            Opcode::StringCapitalize,
            Opcode::LoopMap,
            Opcode::LoopFilter,
            Opcode::LoopFlatMap,
            Opcode::LoopUniqueBy,
            Opcode::LoopGroupBy,
            Opcode::LoopEachToString,
            Opcode::LoopEvery,
            Opcode::LoopSome,
            Opcode::LoopFind,
            Opcode::LoopSort,
            Opcode::LoopCompact,
            Opcode::Throw,
            Opcode::Catch,
            Opcode::CatchEnd,
            Opcode::MakeAsync,
            Opcode::Await,
            Opcode::ReverseArgs,
            Opcode::SilentExec,
            Opcode::Import,
            Opcode::Export,
        ];

        for opcode in all_opcodes {
            let byte = opcode as u8;
            let decoded = Opcode::from_u8(byte);
            assert_eq!(Some(opcode), decoded, "Failed to roundtrip opcode {:?}", opcode);
        }

            assert_eq!(None, Opcode::from_u8(7));
            assert_eq!(None, Opcode::from_u8(13));
            assert_eq!(None, Opcode::from_u8(25));
            assert_eq!(None, Opcode::from_u8(36));
            assert_eq!(None, Opcode::from_u8(46));
            assert_eq!(None, Opcode::from_u8(54));
            assert_eq!(None, Opcode::from_u8(66));
            assert_eq!(None, Opcode::from_u8(76));
            assert_eq!(None, Opcode::from_u8(86));
            assert_eq!(None, Opcode::from_u8(101));
            assert_eq!(None, Opcode::from_u8(113));
            assert_eq!(None, Opcode::from_u8(122));
            assert_eq!(None, Opcode::from_u8(132));
            assert_eq!(None, Opcode::from_u8(142));
            assert_eq!(None, Opcode::from_u8(200));
            assert_eq!(None, Opcode::from_u8(255));
    }

    #[test]
    fn opcode_display() {
        assert_eq!("NOP", format!("{}", Opcode::Nop));
        assert_eq!("RET", format!("{}", Opcode::Return));
        assert_eq!("JMP", format!("{}", Opcode::Jump));
        assert_eq!("JMP_T", format!("{}", Opcode::JumpIfTrue));
        assert_eq!("JMP_F", format!("{}", Opcode::JumpIfFalse));
        assert_eq!("CALL", format!("{}", Opcode::Call));
        assert_eq!("TAIL_CALL", format!("{}", Opcode::TailCall));
        assert_eq!("PUSH", format!("{}", Opcode::Push));
        assert_eq!("POP", format!("{}", Opcode::Pop));
        assert_eq!("DUP", format!("{}", Opcode::Dup));
        assert_eq!("LOAD_CONST", format!("{}", Opcode::LoadConstant));
        assert_eq!("LOAD_REG", format!("{}", Opcode::LoadRegister));
        assert_eq!("STORE_REG", format!("{}", Opcode::StoreRegister));
        assert_eq!("LOAD_LOCAL", format!("{}", Opcode::LoadLocal));
        assert_eq!("STORE_LOCAL", format!("{}", Opcode::StoreLocal));
        assert_eq!("ADD", format!("{}", Opcode::Add));
        assert_eq!("SUB", format!("{}", Opcode::Sub));
        assert_eq!("MUL", format!("{}", Opcode::Mul));
        assert_eq!("DIV", format!("{}", Opcode::Div));
        assert_eq!("MOD", format!("{}", Opcode::Mod));
        assert_eq!("NEG", format!("{}", Opcode::Neg));
        assert_eq!("EQ", format!("{}", Opcode::Eq));
        assert_eq!("NE", format!("{}", Opcode::Ne));
        assert_eq!("LT", format!("{}", Opcode::Lt));
        assert_eq!("LE", format!("{}", Opcode::Le));
        assert_eq!("GT", format!("{}", Opcode::Gt));
        assert_eq!("GE", format!("{}", Opcode::Ge));
        assert_eq!("NOT", format!("{}", Opcode::Not));
        assert_eq!("AND", format!("{}", Opcode::And));
        assert_eq!("OR", format!("{}", Opcode::Or));
        assert_eq!("TO_BOOL", format!("{}", Opcode::ToBoolean));
        assert_eq!("MAP_NEW", format!("{}", Opcode::MapNew));
        assert_eq!("MAP_GET", format!("{}", Opcode::MapGet));
        assert_eq!("MAP_SET", format!("{}", Opcode::MapSet));
        assert_eq!("MAP_MERGE", format!("{}", Opcode::MapMerge));
        assert_eq!("MAP_PICK", format!("{}", Opcode::MapPick));
        assert_eq!("MAP_OMIT", format!("{}", Opcode::MapOmit));
        assert_eq!("LIST_NEW", format!("{}", Opcode::ListNew));
        assert_eq!("LIST_GET", format!("{}", Opcode::ListGet));
        assert_eq!("LIST_SET", format!("{}", Opcode::ListSet));
        assert_eq!("LIST_PUSH", format!("{}", Opcode::ListPush));
        assert_eq!("LIST_POP", format!("{}", Opcode::ListPop));
        assert_eq!("LIST_LEN", format!("{}", Opcode::ListLen));
        assert_eq!("STR_CAT", format!("{}", Opcode::StringConcat));
        assert_eq!("STR_LEN", format!("{}", Opcode::StringLen));
        assert_eq!("STR_SLICE", format!("{}", Opcode::StringSlice));
        assert_eq!("STR_UPPER", format!("{}", Opcode::StringToUpper));
        assert_eq!("STR_LOWER", format!("{}", Opcode::StringToLower));
        assert_eq!("STR_CAP", format!("{}", Opcode::StringCapitalize));
        assert_eq!("LOOP_MAP", format!("{}", Opcode::LoopMap));
        assert_eq!("LOOP_FILTER", format!("{}", Opcode::LoopFilter));
        assert_eq!("LOOP_FLATMAP", format!("{}", Opcode::LoopFlatMap));
        assert_eq!("LOOP_UNIQUE", format!("{}", Opcode::LoopUniqueBy));
        assert_eq!("LOOP_GROUP", format!("{}", Opcode::LoopGroupBy));
        assert_eq!("LOOP_EACH_STR", format!("{}", Opcode::LoopEachToString));
        assert_eq!("LOOP_EVERY", format!("{}", Opcode::LoopEvery));
        assert_eq!("LOOP_SOME", format!("{}", Opcode::LoopSome));
        assert_eq!("LOOP_FIND", format!("{}", Opcode::LoopFind));
        assert_eq!("LOOP_SORT", format!("{}", Opcode::LoopSort));
        assert_eq!("LOOP_COMPACT", format!("{}", Opcode::LoopCompact));
        assert_eq!("THROW", format!("{}", Opcode::Throw));
        assert_eq!("CATCH", format!("{}", Opcode::Catch));
        assert_eq!("CATCH_END", format!("{}", Opcode::CatchEnd));
        assert_eq!("MAKE_ASYNC", format!("{}", Opcode::MakeAsync));
        assert_eq!("AWAIT", format!("{}", Opcode::Await));
        assert_eq!("REVERSE_ARGS", format!("{}", Opcode::ReverseArgs));
        assert_eq!("SILENT_EXEC", format!("{}", Opcode::SilentExec));
        assert_eq!("IMPORT", format!("{}", Opcode::Import));
        assert_eq!("EXPORT", format!("{}", Opcode::Export));
    }

    #[test]
    fn instruction_display() {
        let inst = Instruction::new(Opcode::Add, 1, 2, 3);
        assert_eq!("ADD 1 2 3", format!("{}", inst));

        let inst = Instruction::new(Opcode::Call, 10, 20, 30);
        assert_eq!("CALL 10 20 30", format!("{}", inst));

        let inst = Instruction::new(Opcode::LoadConstant, 0, 5, 0);
        assert_eq!("LOAD_CONST 0 5 0", format!("{}", inst));
    }
}