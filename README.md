# Alic
Proof of concept tree-sitter based transpiler for improved Microsoft AL programming language. Part of my bachelor's thesis, where I proved that it is possible to write a fast transpiler (when compared to other transpilers) using tree-sitter parser.

NOTE: Syntax error detection is not yet provided -- the parser will just recover and the transpiler will not notify user in any way (but the output may be broken).

This transpiler compiles Ali (AL Improved) programming language to AL programming language. The former is similar in syntax to JavaScript and the latter is similar in syntax to Pascal.

The parser resides in [tree-sitter-ali folder](./tree-sitter-ali).

The transpiler itself is written in Rust.

## Usage
`./alic <input-file> <output-file>`

Both arguments are optional. Default input is `input` file and default output is stdout.

Example input is in [input file](./input) and the output for it is in [output file](./output).

