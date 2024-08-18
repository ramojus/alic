use super::{
    constants::{content, node_kind},
    walker::ContextType,
    walker::Walker,
};

#[derive(Debug)]
pub struct Customizer<'source> {
    var_declarations_pos: usize,
    newline_whitespace_at_var_declarations: &'source [u8],
    saved_var_declarations: Vec<Vec<u8>>,
}

impl<'source> Customizer<'source> {
    pub fn new() -> Self {
        Self {
            var_declarations_pos: 0,
            newline_whitespace_at_var_declarations: &[],
            saved_var_declarations: vec![],
        }
    }

    pub fn match_node(walker: &mut Walker<'source>) -> bool {
        let mut have_matched = true;
        let node = walker.cursor.node();
        let mut map = |new_content| walker.step_mapped_node(new_content);
        match node.kind() {
            // general
            node_kind::MULTIPLE_STATEMENTS => Self::write_wrapped_in_block(walker),
            node_kind::CODE_BEGIN => Self::write_begin(walker),
            node_kind::CODE_END => Self::write_end(walker),

            // operators
            node_kind::ASSIGN_OP => Self::write_assign_op(walker),
            node_kind::MOD_OP => map(content::MOD_OP),
            node_kind::EQUALS_OP => map(content::EQUALS_OP),
            node_kind::NOT_EQUALS_OP => map(content::NOT_EQUALS_OP),

            // misc
            node_kind::METHOD_DEFINITION => Self::write_with_semicolon(walker),
            node_kind::IF_STATEMENT => Self::write_control_flow_statement(walker, &content::THEN),
            node_kind::WHILE_LOOP => Self::write_control_flow_statement(walker, &content::DO),
            node_kind::REPEAT_UNTIL_LOOP => Self::write_repeat_until_loop(walker),
            node_kind::FOR_LOOP => Self::write_control_flow_statement(walker, &content::DO),
            node_kind::FOREACH_LOOP => Self::write_control_flow_statement(walker, &content::DO),
            node_kind::BINARY_EXPRESSION => Self::write_binary_expression(walker),

            // return statement
            node_kind::RETURN_STATEMENT => Self::write_return_statement(walker, false),
            node_kind::RETURN_STATEMENT_WITH_EXPRESSION => Self::write_return_statement(walker, true),
            node_kind::RETURN_KEYWORD => map(content::EXIT),

            // switch statement
            node_kind::SWITCH_STATEMENT => Self::write_switch_statement(walker),
            node_kind::SWITCH_KEYWORD => map(content::CASE),
            node_kind::SWITCH_BEGIN => map(content::OF),
            node_kind::SWITCH_END => map(content::SEPARATED_END),
            node_kind::CASE_KEYWORD => map(b""),
            node_kind::CASE_DEFAULT_KEYWORD => map(content::ELSE),
            node_kind::CASE_DEFAULT_BEGIN => map(b""),

            // vars
            node_kind::VAR_DECLARATION => Self::save_var_declaration(walker, false),
            node_kind::VAR_DECLARATION_WITH_ASSIGNMENT => Self::save_var_declaration(walker, true),
            _ => {
                have_matched = false;
            }
        }
        have_matched
    }

    pub fn clear_saved_vars(&mut self) {
        self.var_declarations_pos = 0;
        self.newline_whitespace_at_var_declarations = &[];
        self.saved_var_declarations.clear();
    }

    fn write_begin(walker: &mut Walker) {
        let parent_context = walker.context.get_parent_context();

        match parent_context {
            Some(ContextType::Method) => {
                walker.customizer.var_declarations_pos = walker.writer.get_output_len();
                walker.writer.write_new_line_content(content::BEGIN);
                walker.customizer.newline_whitespace_at_var_declarations = walker.writer.current_newline_whitespace;
            }
            _ => {
                walker.step_mapped_node(content::BEGIN);
            }
        }
        walker.reader.skip_content(&walker.cursor.node());
    }

    fn write_end(walker: &mut Walker) {
        let parent_context = walker.context.get_parent_context();
        walker.step_mapped_node(content::END);

        if parent_context == Some(ContextType::Method) {
            Self::write_var_declarations(walker);
        }
    }

    fn write_assign_op(walker: &mut Walker) {
        if walker.context.get_current_context() == Some(ContextType::Code) {
            walker.step_mapped_node(content::ASSIGN_OP);
        } else {
            walker.walk_node(false);
        }
    }

    fn write_control_flow_statement(walker: &mut Walker<'source>, begin_keyword: &'static [u8]) {
        walker.skip_to_first_child();
        walker.try_walk_to_sibling(node_kind::CODE_BLOCK);
        walker.writer.chunk.leading_whitespace = b" ";
        walker.writer.chunk.content = begin_keyword;
        walker.writer.write_chunk();
        walker.walk();

        walker.skip_to_parent();
        walker.writer.set_prev_chunk_append(content::STATEMENT_SEPARATOR);
    }

    fn write_repeat_until_loop(walker: &mut Walker) {
        walker.skip_to_first_child();
        let code_end_leading_whitespace;
        walker.try_walk_to_sibling(node_kind::CODE_BLOCK);
        {
            walker.skip_to_first_child();
            walker.try_walk_to_sibling(node_kind::CODE_BEGIN);
            walker.reader.skip_content(&walker.cursor.node());
            walker.skip_to_next_sibling();

            walker.try_walk_to_sibling(node_kind::CODE_END);
            code_end_leading_whitespace = walker.reader.read_leading_whitespace(&walker.cursor.node());
            walker.reader.skip_content(&walker.cursor.node());

            walker.skip_to_parent();
        }
        walker.skip_to_next_sibling();
        let content_after_code_end = walker.reader.read_content(&walker.cursor.node());
        walker.writer.chunk.leading_whitespace = code_end_leading_whitespace;
        walker.writer.chunk.content = content_after_code_end;
        walker.writer.write_chunk();
        walker.reader.skip_content(&walker.cursor.node());
        walker.skip_to_next_node();

        walker.walk();
        walker.skip_to_parent();
    }

    fn write_binary_expression(walker: &mut Walker) {
        let parent_context = walker.context.get_parent_context();
        if parent_context.is_some_and(|context| context == ContextType::BinaryExpr) {
            Self::write_wrapped_in_parenthesis(walker);
        } else {
            walker.walk_node(false);
        }
    }

    fn write_return_statement(walker: &mut Walker, is_with_expression: bool) {
        walker.skip_to_first_child();

        if walker.try_walk_to_sibling(node_kind::RETURN_KEYWORD) {
            walker.walk_node(true);
            walker.skip_to_next_sibling();
        }

        if is_with_expression {
            walker.reader.skip_content(&walker.cursor.node());
            walker.writer.chunk.set_prepend(b"(");
            walker.try_walk_to_sibling(node_kind::EXPRESSION);
            walker.walk_node(true);
            walker.skip_to_next_sibling();
            walker.writer.set_prev_chunk_append(b")");
        } else {
            walker.try_walk_to_sibling(node_kind::STATEMENT_END);
        }

        walker.walk();
        walker.skip_to_parent();
    }

    fn write_switch_statement(walker: &mut Walker) {
        if !walker.skip_to_first_child() {
            panic!("no children for switch statement")
        }
        walker.try_walk_to_sibling(node_kind::SWITCH_BEGIN);
        walker.writer.push_indent();
        walker.try_walk_to_sibling(node_kind::SWITCH_END);
        walker.writer.pop_indent();

        walker.walk();
        walker.skip_to_parent();
    }

    fn save_var_declaration(walker: &mut Walker, is_also_assignment: bool) {
        walker.skip_to_first_child();

        walker.writer.set_write_to_temp_output(true);
        walker.writer.set_write_to_normal_output(is_also_assignment);

        walker.try_walk_to_sibling(node_kind::DECLARATION_KEYWORD);
        if is_also_assignment {
            walker.writer.set_write_to_temp_output(false);
            walker.step_mapped_node(b"");
            walker.writer.set_write_to_temp_output(true);
        }
        walker.skip_to_next_sibling();
        walker.reader.skip_content(&walker.cursor.node());

        if is_also_assignment {
            walker.try_walk_to_sibling(node_kind::VAR_TYPE_DECLARATION);
            walker.writer.set_write_to_normal_output(false);
            walker.walk_node(false);
            walker.skip_to_next_sibling();

            walker.writer.set_write_to_normal_output(true);
            walker.writer.set_write_to_temp_output(false);
            walker.try_walk_to_sibling(node_kind::STATEMENT_END);
            walker.writer.set_write_to_temp_output(true);
        }

        walker.walk();
        walker.skip_to_parent();

        let var_declaration = walker.writer.get_temp_output();
        walker.writer.set_write_to_temp_output(false);
        walker.writer.set_write_to_normal_output(true);

        walker.customizer.saved_var_declarations.push(var_declaration);
    }

    fn write_var_declarations(walker: &mut Walker) {
        if walker.customizer.saved_var_declarations.len() == 0 {
            return;
        }

        let newline_whitespace = walker.customizer.newline_whitespace_at_var_declarations;

        let mut content = [newline_whitespace, content::VAR].concat().to_vec();
        content.extend::<Vec<u8>>(walker.customizer.saved_var_declarations.iter().fold(
            vec![],
            |mut decl_content, declaration| {
                decl_content.extend(newline_whitespace);
                decl_content.extend(content::INDENT);
                decl_content.extend(declaration);
                decl_content
            },
        ));
        walker
            .writer
            .insert_to_output(walker.customizer.var_declarations_pos, &content);
        walker.customizer.clear_saved_vars();
    }

    fn write_with_semicolon(walker: &mut Walker) {
        walker.walk_node(false);
        walker.writer.set_prev_chunk_append(b";");
    }

    fn write_wrapped_in_parenthesis(walker: &mut Walker) {
        walker.writer.chunk.set_prepend(b"(");
        walker.walk_node(false);
        walker.writer.set_prev_chunk_append(b")");
    }

    fn write_wrapped_in_block(walker: &mut Walker) {
        walker.writer.chunk.set_prepend_line_content(content::BEGIN);
        walker.writer.push_indent();
        walker.walk_node(false);
        walker.writer.pop_indent();
        walker.writer.write_new_line_content(content::SEPARATED_END);
    }
}
