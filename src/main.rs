use std::{env, fs::{self, read}};

use crate::compiler::walker::Walker;
use tree_sitter::Parser;

pub mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ali::language())
        .expect("Error loading Ali grammar");

    let source_path = match args.get(1) {
        Some(arg) => arg,
        None => "input",
    };
    let source = read(source_path).unwrap();
    let tree = parser.parse(&source, None).unwrap();

    let cursor = tree.walk();
    let mut walker = Walker::new(cursor, &source);
    walker.walk();
    let output = walker.writer.get_output();

    let output_path = args.get(2);
    match output_path {
        Some(path) => fs::write(path, output).unwrap(),
        None => println!("{}", String::from_utf8(output.to_vec()).unwrap()),
    };
}
