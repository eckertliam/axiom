mod ast;
mod parser;
mod ir;
mod ir_builder;

use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let program = parser::parse(&file_content);
    let module = ir_builder::build_ir(program);
}
