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
    MapBuild = 66,

    ListNew = 70,
    ListGet = 71,
    ListSet = 72,
    ListPush = 73,
    ListPop = 74,
    ListLen = 75,
    ListBuild = 76,

    StringConcat = 80,
    StringLen = 81,


    LoopEachToString = 95,
    Throw = 110,
    Catch = 111,
    CatchEnd = 112,

    MakeAsync = 120,
    Await = 121,

    ReverseArgs = 130,
    Import = 140,
    Export = 141,
    BindName = 142,
    LookupName = 143,
    MakeClosure = 150,
    LoadArg = 151,
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
            66 => Some(Opcode::MapBuild),

            70 => Some(Opcode::ListNew),
            71 => Some(Opcode::ListGet),
            72 => Some(Opcode::ListSet),
            73 => Some(Opcode::ListPush),
            74 => Some(Opcode::ListPop),
            75 => Some(Opcode::ListLen),
            76 => Some(Opcode::ListBuild),

            80 => Some(Opcode::StringConcat),
            81 => Some(Opcode::StringLen),
            95 => Some(Opcode::LoopEachToString),

            110 => Some(Opcode::Throw),
            111 => Some(Opcode::Catch),
            112 => Some(Opcode::CatchEnd),

            120 => Some(Opcode::MakeAsync),
            121 => Some(Opcode::Await),

            130 => Some(Opcode::ReverseArgs),
            140 => Some(Opcode::Import),
            141 => Some(Opcode::Export),
            142 => Some(Opcode::BindName),
            143 => Some(Opcode::LookupName),
            150 => Some(Opcode::MakeClosure),
            151 => Some(Opcode::LoadArg),

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
            Opcode::MapBuild => write!(f, "MAP_BUILD"),

            Opcode::ListNew => write!(f, "LIST_NEW"),
            Opcode::ListGet => write!(f, "LIST_GET"),
            Opcode::ListSet => write!(f, "LIST_SET"),
            Opcode::ListPush => write!(f, "LIST_PUSH"),
            Opcode::ListPop => write!(f, "LIST_POP"),
            Opcode::ListLen => write!(f, "LIST_LEN"),
            Opcode::ListBuild => write!(f, "LIST_BUILD"),

            Opcode::StringConcat => write!(f, "STR_CAT"),
            Opcode::StringLen => write!(f, "STR_LEN"),
            Opcode::LoopEachToString => write!(f, "LOOP_EACH_STR"),

            Opcode::Throw => write!(f, "THROW"),
            Opcode::Catch => write!(f, "CATCH"),
            Opcode::CatchEnd => write!(f, "CATCH_END"),

            Opcode::MakeAsync => write!(f, "MAKE_ASYNC"),
            Opcode::Await => write!(f, "AWAIT"),

            Opcode::ReverseArgs => write!(f, "REVERSE_ARGS"),
            Opcode::Import => write!(f, "IMPORT"),
            Opcode::Export => write!(f, "EXPORT"),
            Opcode::BindName => write!(f, "BIND_NAME"),
            Opcode::LookupName => write!(f, "LOOKUP_NAME"),
            Opcode::MakeClosure => write!(f, "MAKE_CLOSURE"),
            Opcode::LoadArg => write!(f, "LOAD_ARG"),
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
            Opcode::MapBuild,
            Opcode::ListNew,
            Opcode::ListGet,
            Opcode::ListSet,
            Opcode::ListPush,
            Opcode::ListPop,
            Opcode::ListLen,
            Opcode::ListBuild,
            Opcode::StringConcat,
            Opcode::StringLen,
            Opcode::LoopEachToString,
            Opcode::Throw,
            Opcode::Catch,
            Opcode::CatchEnd,
            Opcode::MakeAsync,
            Opcode::Await,
            Opcode::ReverseArgs,
            Opcode::Import,
            Opcode::Export,
            Opcode::BindName,
            Opcode::LookupName,
            Opcode::MakeClosure,
            Opcode::LoadArg,
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
            assert_eq!(None, Opcode::from_u8(67));
            assert_eq!(None, Opcode::from_u8(86));
            assert_eq!(None, Opcode::from_u8(101));
            assert_eq!(None, Opcode::from_u8(113));
            assert_eq!(None, Opcode::from_u8(122));
            assert_eq!(None, Opcode::from_u8(132));
            assert_eq!(None, Opcode::from_u8(144));
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
        assert_eq!("LIST_NEW", format!("{}", Opcode::ListNew));
        assert_eq!("LIST_GET", format!("{}", Opcode::ListGet));
        assert_eq!("LIST_SET", format!("{}", Opcode::ListSet));
        assert_eq!("LIST_PUSH", format!("{}", Opcode::ListPush));
        assert_eq!("LIST_POP", format!("{}", Opcode::ListPop));
        assert_eq!("LIST_LEN", format!("{}", Opcode::ListLen));
        assert_eq!("STR_CAT", format!("{}", Opcode::StringConcat));
        assert_eq!("STR_LEN", format!("{}", Opcode::StringLen));
        assert_eq!("LOOP_EACH_STR", format!("{}", Opcode::LoopEachToString));
        assert_eq!("THROW", format!("{}", Opcode::Throw));
        assert_eq!("CATCH", format!("{}", Opcode::Catch));
        assert_eq!("CATCH_END", format!("{}", Opcode::CatchEnd));
        assert_eq!("MAKE_ASYNC", format!("{}", Opcode::MakeAsync));
        assert_eq!("AWAIT", format!("{}", Opcode::Await));
        assert_eq!("REVERSE_ARGS", format!("{}", Opcode::ReverseArgs));
        assert_eq!("IMPORT", format!("{}", Opcode::Import));
        assert_eq!("EXPORT", format!("{}", Opcode::Export));
        assert_eq!("BIND_NAME", format!("{}", Opcode::BindName));
        assert_eq!("LOOKUP_NAME", format!("{}", Opcode::LookupName));
        assert_eq!("MAKE_CLOSURE", format!("{}", Opcode::MakeClosure));
        assert_eq!("LOAD_ARG", format!("{}", Opcode::LoadArg));
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