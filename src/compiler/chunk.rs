#[derive(Debug)]
pub struct Chunk<'source> {
    pub content: &'source [u8],
    pub leading_whitespace: &'source [u8],
    pub prepend_line_content: Vec<u8>,
    prepend: Vec<u8>,
}

impl<'source> Chunk<'source> {
    pub fn new(content: &'source [u8], leading_whitespace: &'source [u8]) -> Self {
        Self {
            content,
            leading_whitespace,
            prepend: Vec::new(),
            prepend_line_content: Vec::new(),
        }
    }

    pub fn new_empty() -> Self {
        Self::new(&[], &[])
    }

    pub fn set_prepend(&mut self, prepend: &'source [u8]) {
        self.prepend = [prepend, &self.prepend].concat();
    }

    pub fn get_prepend(&self) -> &[u8] {
        &self.prepend
    }

    pub fn clear_prepend(&mut self) {
        self.prepend.clear();
    }

    pub fn set_prepend_line_content(&mut self, line: &[u8]) {
        self.prepend_line_content = line.to_vec();
    }
}
