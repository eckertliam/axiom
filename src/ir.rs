use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Opcode {
    Jump(u64),
    // JumpIf(block_index, block_index_if_false)
    JumpIf(u64, u64),
    Ret,

    // Integer instructions
    // push an immediate integer
    IPush(i64),
    // pop a pointer from the stack and load the integer at that address
    ILoad,
    // store an int and push its address to the stack
    IStore,
    IAdd,
    ISub,
    IMul,
    IDiv,
    IMod,
    IPow,
    IComp,
    // print the top of the stack as an integer
    IPrint,

    // Float instructions
    // push an immediate float
    FPush(f64),
    // pop a pointer from the stack and load the float at that address
    FLoad,
    // store a float and push its address to the stack
    FStore,
    FAdd,
    FSub,
    FMul,
    FDiv,
    FComp,
    // print the top of the stack as a float
    FPrint,

    // Pointer instructions
    // push an immediate pointer
    PPush(u64),
    // pop a pointer from the stack and load the pointer at that address
    PLoad,
    // store a pointer and push its address to the stack
    PStore,
    PAdd,
    PSub,
    PMul,
    PDiv,
    PMod,
    PComp,
    // print the top of the stack as a pointer
    PPrint,

    // Boolean instructions
    // push an immediate boolean
    BPush(bool),
    // pop a pointer from the stack and load the boolean at that address
    BLoad,
    // store a boolean and push its address to the stack
    BStore,
    // logical and the top two booleans on the stack
    BAnd,
    // logical or the top two booleans on the stack
    BOr,
    // logical not the top boolean on the stack
    BNot,
    // print the top of the stack as a boolean
    BPrint,

    // Character instructions
    // push an immediate character
    CPush(char),
    // pop a pointer from the stack and load the character at that address
    CLoad,
    // store a character and push its address to the stack
    CStore,
    CComp,
    // print the top of the stack as a character
    CPrint,
}

pub struct Block {
    pub label: String,
    pub ops: Vec<Opcode>,
}

pub struct Module {
    pub blocks: Vec<Block>,
    pub block_map: HashMap<String, usize>,
    pub current_block: usize,
}

impl Module {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block {
                label: "entry".to_string(),
                ops: Vec::new(),
            }],
            block_map: HashMap::from([("entry".to_string(), 0)]),
            current_block: 0,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.block_map.insert(block.label.clone(), self.blocks.len());
        self.blocks.push(block);
    }

    pub fn get_block(&self, label: &str) -> Option<usize> {
        self.block_map.get(label).cloned()
    }

    pub fn set_current_block(&mut self, label: &str) {
        self.current_block = self.get_block(label).unwrap();
    }

    pub fn add_op(&mut self, op: Opcode) {
        self.blocks[self.current_block].ops.push(op);
    }
}
