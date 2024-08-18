use tree_sitter::Parser;
use alic::compiler::walker::Walker;
use std::str;

pub fn test_writer_output_is_target(source: &[u8], target: &[u8]) {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ali::language())
        .expect("Error loading Ali grammar");

    let source = source.to_vec();
    let target = target.to_vec();

    let tree = parser.parse(&source, None).unwrap();
    let cursor = tree.walk();
    let mut writer = Walker::new(cursor, &source);
    writer.walk();
    let output = writer.writer.get_output();

    // println!("{}", str::from_utf8(&writer.get_output()).unwrap());
    if *output != target {
        panic!(
            "output not equal to target.\noutput: [{}]\ntarget: [{}]",
            str::from_utf8(output).unwrap(),
            str::from_utf8(&target).unwrap()
        );
    }
    assert!(*output == target);
}
