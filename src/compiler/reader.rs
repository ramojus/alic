use tree_sitter::Node;

#[derive(Debug)]
pub struct Reader<'source> {
    source: &'source [u8],
    cursor_pos: usize,
}

impl<'source> Reader<'source> {
    pub fn new(source: &'source [u8]) -> Self {
        Self {
            source,
            cursor_pos: 0,
        }
    }

    pub fn read_leading_whitespace(&mut self, node: &Node) -> &'source [u8] {
        let leading_whitespace = self.source
            .get(self.cursor_pos..node.start_byte())
            .unwrap_or(&[]);
        self.cursor_pos = node.start_byte();

        leading_whitespace
    }

    pub fn read_content(&mut self, node: &Node) -> &'source [u8] {
        let content = self.source
            .get(node.byte_range())
            .expect(&format!("no content for node {}", node));
        self.cursor_pos = node.end_byte();

        content
    }

    pub fn skip_content(&mut self, node: &Node) {
        self.cursor_pos = node.end_byte();
    }

    pub fn read_rest_of_source(&mut self) -> &'source [u8] {
        self.source.get(self.cursor_pos..).unwrap()
    }

}
