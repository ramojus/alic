use super::{constants::node_kind, customizer::Customizer, reader::Reader, writer::Writer};
use std::collections::HashMap;
use tree_sitter::TreeCursor;

pub struct Walker<'cursor> {
    pub cursor: TreeCursor<'cursor>,
    pub writer: Writer<'cursor>,
    pub reader: Reader<'cursor>,
    pub context: Context<'cursor>,
    pub customizer: Customizer<'cursor>,
}

#[derive(Debug)]
pub struct Context<'a> {
    context: Vec<ContextType>,
    mappings: HashMap<&'a str, ContextType>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContextType {
    Code,
    BinaryExpr,
    ParenthesizedExpr,
    Method,
}

impl Context<'_> {
    pub fn new() -> Self {
        Self {
            context: Vec::new(),
            mappings: HashMap::from([
                ("code_block", ContextType::Code),
                ("binary_expression", ContextType::BinaryExpr),
                ("parenthesized_expression", ContextType::ParenthesizedExpr),
                ("method_definition", ContextType::Method),
            ]),
        }
    }

    pub fn push_context(&mut self, node_type: &str) {
        if let Some(context) = self.mappings.get(node_type) {
            self.context.push(*context);
        }
    }

    pub fn pop_context(&mut self, node_type: &str) {
        if self.mappings.get(node_type).is_some() {
            self.context.pop();
        }
    }

    pub fn get_parent_context(&self) -> Option<ContextType> {
        if let Some(parent_index) = self.context.len().checked_sub(2) {
            self.context.get(parent_index).copied()
        } else {
            None
        }
    }

    pub fn get_current_context(&self) -> Option<ContextType> {
        self.context.last().copied()
    }
}

impl Default for Context<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'cursor> Walker<'cursor> {
    pub fn new(cursor: TreeCursor<'cursor>, source: &'cursor Vec<u8>) -> Self {
        Self {
            cursor,
            writer: Writer::new(source.len()),
            reader: Reader::new(source),
            context: Context::new(),
            customizer: Customizer::new(),
        }
    }

    pub fn walk(&mut self) {
        self.walk_node(true);
        while self.skip_to_next_sibling() {
            self.walk();
        }
        if self.cursor.node().kind() == node_kind::ROOT {
            self.writer.extend_output(self.reader.read_rest_of_source());
        }
    }

    pub fn walk_node(&mut self, customize_node: bool) {
        if customize_node && Customizer::match_node(self) {
            return;
        }
        if self.skip_to_first_child() {
            self.walk();
            self.skip_to_parent();
        } else {
            self.step_node();
        }
    }

    fn step_node(&mut self) {
        self.writer.chunk.leading_whitespace = self.reader.read_leading_whitespace(&self.cursor.node());
        self.writer.chunk.content = self.reader.read_content(&self.cursor.node());
        self.writer.write_chunk();
    }

    pub fn step_mapped_node(&mut self, new_content: &'cursor [u8]) {
        self.writer.chunk.leading_whitespace = self.reader.read_leading_whitespace(&self.cursor.node());
        self.writer.chunk.content = new_content;
        self.writer.write_chunk();

        self.reader.skip_content(&self.cursor.node());
    }

    pub fn skip_to_next_sibling(&mut self) -> bool {
        let prev_context = self.cursor.node().kind();
        if self.cursor.goto_next_sibling() {
            self.context.pop_context(prev_context);
            self.context.push_context(self.cursor.node().kind());
            return true;
        }
        false
    }

    pub fn skip_to_first_child(&mut self) -> bool {
        if self.cursor.goto_first_child() {
            self.context.push_context(self.cursor.node().kind());
            return true;
        }
        false
    }

    pub fn skip_to_parent(&mut self) -> bool {
        self.context.pop_context(self.cursor.node().kind());
        self.cursor.goto_parent()
    }

    pub fn skip_to_next_node(&mut self) -> bool {
        if self.skip_to_next_sibling() {
            return true;
        }
        if self.skip_to_parent() {
            self.skip_to_next_node()
        } else {
            false
        }
    }

    pub fn try_walk_to_sibling(&mut self, sibling_type: &str) -> bool {
        if self.cursor.node().kind() == sibling_type {
            return true;
        } else {
            self.walk_node(true);
        }
        while self.skip_to_next_sibling() {
            if self.cursor.node().kind() == sibling_type {
                return true;
            }
            self.walk_node(true);
        }
        false
    }
}
