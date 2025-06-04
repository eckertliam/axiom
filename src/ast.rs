use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Symbol(String),
    Integer(i64),
    Float(f64),
    Pointer(u64),
    Char(char),
    Boolean(bool),
    Null, // not an actual value, but a placeholder for no value
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Pointer(p) => write!(f, "{}", p),
            Value::Char(c) => write!(f, "{}", c),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub struct Instruction {
    pub instr: String,
    pub args: [Value; 3],
}

impl Instruction {
    pub fn new(instr: String, args: [Value; 3]) -> Self {
        Self { instr, args }
    }
}

pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Self {
        Self { instructions: Vec::new() }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}