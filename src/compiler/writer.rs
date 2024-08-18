use super::chunk::Chunk;
use std::mem;

use super::constants::content;

#[derive(Debug)]
pub struct Writer<'source> {
    output: Vec<u8>,
    save_to_output: bool,
    temp_output: Vec<u8>,
    save_to_temp_output: bool,
    pub chunk: Chunk<'source>,
    prev_chunk_append: Vec<u8>,
    additional_indent: usize,
    pub current_newline_whitespace: &'source [u8],
    should_ignore_next_leading_whitespace: bool,
}

impl<'source> Writer<'source> {
    pub fn new(source_len: usize) -> Self {
        Self {
            output: Vec::with_capacity(source_len),
            save_to_output: true,
            temp_output: Vec::new(),
            save_to_temp_output: false,
            chunk: Chunk::new_empty(),
            prev_chunk_append: Vec::new(),
            additional_indent: 0,
            current_newline_whitespace: &[],
            should_ignore_next_leading_whitespace: false,
        }
    }

    pub fn set_write_to_temp_output(&mut self, should_save: bool) {
        self.save_to_temp_output = should_save;
    }

    pub fn set_write_to_normal_output(&mut self, should_save: bool) {
        self.save_to_output = should_save;
    }

    pub fn get_temp_output(&mut self) -> Vec<u8> {
        mem::take(&mut self.temp_output)
    }

    pub fn set_prev_chunk_append(&mut self, append: &'source [u8]) {
        self.prev_chunk_append.append(&mut append.to_vec());
    }

    pub fn write_chunk(&mut self) {
        let leading_whitespace = if self.should_ignore_next_leading_whitespace {
            self.should_ignore_next_leading_whitespace = false;
            b""
        } else {
            self.chunk.leading_whitespace
        };

        if self.try_save_newline_whitespace(&leading_whitespace) {
            let indented_newline = self.get_indented(leading_whitespace);
            self.extend_output_with_chunk(&indented_newline, self.chunk.content);
        } else {
            self.extend_output_with_chunk(&leading_whitespace, self.chunk.content);
        }

        self.should_ignore_next_leading_whitespace = self.chunk.content == b"" && leading_whitespace != b"";
        self.chunk = Chunk::new_empty();
    }

    pub fn try_save_newline_whitespace(&mut self, leading_whitespace: &'source [u8]) -> bool {
        let newline_index = match leading_whitespace
            .iter()
            .zip(0..leading_whitespace.len())
            .rev() // to find the latest newline
            .find(|(&e, _)| e == b'\n')
        {
            Some((_, index)) => Some(index),
            None => None,
        };

        if let Some(newline_index) = newline_index {
            self.current_newline_whitespace = &leading_whitespace[newline_index..];
            true
        } else {
            false
        }
    }

    pub fn write_new_line_content(&mut self, content: &'source [u8]) {
        let indented_newline = self.get_indented(self.current_newline_whitespace);
        self.extend_output_with_chunk(&indented_newline, content);
    }

    fn extend_output_with_chunk(&mut self, leading_indented_whitespace: &[u8], content: &[u8]) {
        let prepend_line = self.get_prepend_line();
        let extend_content = [
            &self.prev_chunk_append,
            &prepend_line,
            leading_indented_whitespace,
            &self.chunk.get_prepend(),
            content,
        ]
        .concat();
        if self.save_to_output {
            self.output.extend_from_slice(&extend_content);
        }
        if self.save_to_temp_output {
            self.temp_output.extend_from_slice(&extend_content);
        }
        self.prev_chunk_append.clear();
        self.chunk.clear_prepend();
    }

    fn get_prepend_line(&mut self) -> Vec<u8> {
        let mut prepend_line = vec![];
        if !&self.chunk.prepend_line_content.is_empty() {
            prepend_line.extend_from_slice(self.current_newline_whitespace);
            prepend_line.extend_from_slice(&content::INDENT.repeat(self.additional_indent - 1));
            prepend_line.extend_from_slice(&self.chunk.prepend_line_content);
        }
        prepend_line
    }

    fn get_indented(&mut self, whitespace: &[u8]) -> Vec<u8> {
        let mut indented_whitespace = whitespace.to_owned();
        indented_whitespace.extend_from_slice(&content::INDENT.repeat(self.additional_indent));
        indented_whitespace
    }

    pub fn insert_to_output(&mut self, index: usize, content: &[u8]) {
        self.output.splice(index..index, content.to_owned());
    }

    pub fn extend_output(&mut self, content: &'source [u8]) {
        self.output.extend_from_slice(content);
    }

    pub fn push_indent(&mut self) {
        self.additional_indent += 1;
    }

    pub fn pop_indent(&mut self) {
        self.additional_indent -= 1;
    }

    pub fn get_newline_whitespace(&self) -> &'source [u8] {
        self.current_newline_whitespace
    }

    pub fn get_output_len(&self) -> usize {
        self.output.len()
    }

    pub fn get_output(&self) -> &Vec<u8> {
        &self.output
    }
}
