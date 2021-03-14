use bytes::{BytesMut, BufMut};

pub struct LexicalAnalyzer {
    current_line: u32,
    buffer: [u8]
}

impl LexicalAnalyzer {
    pub fn new(input: &String) -> Self {
        LexicalAnalyzer {
            current_line: 1,
            buffer: input.as_bytes()
        }
    }

    // Removal of single and multi line comments
    pub fn remove_comments(mut self) {
        let mut i = 0;
        //let mut j = 0;
        let sentinel = b'\0';

        let mut buffer = BytesMut::with_capacity(self.buffer.len() + 1);
        buffer.extend_from_slice(self.buffer);
        buffer.put(b'\0');

        while buffer[i] != sentinel {
            if buffer[i] as char == '\n' {
                self.current_line = self.current_line + 1;
            }
            println!("Current byte: {}", buffer[i]);
            println!("Current character: {}", buffer[i] as char);
            println!("Current line: {}", self.current_line);
            println!("===");
            i = i + 1;
        }
    }
/*
    // Compaction of consecutive white space characters
    fn compact_white_space(buffer: &[u8]) {

    }

    fn is_keyword(word: &str) -> bool {
        return true;
    }

    fn is_operator(word: &str) -> bool {
        return true;
    }

    fn is_symbol(word: &str) -> bool {
        return true;
    }

    fn is_literal(word: &str) -> bool {
        return true;
    }

    fn is_constant(word: &str) -> bool {
        return true;
    }*/
}