enum Ignore {
    SingleLine,
    MultiLine,
    No
}

pub struct LexicalAnalyzer {
    current_line: u32,
    ignore_input: Ignore,
    restart_input: bool,
    buffer: Vec<u8>
}

fn is_whitespace(byte: &u8) -> bool {
    let whitespace_characters = [b'\n', b'\t', b' '];

    return whitespace_characters.contains(byte);
}

impl LexicalAnalyzer {
    pub fn new(input: &String) -> Self {
        let mut byte_buffer: Vec<u8> = Vec::new();

        for byte in input.as_bytes().to_owned() {
            byte_buffer.push(byte);
        }

        LexicalAnalyzer {
            current_line: 1,
            ignore_input: Ignore::No,
            restart_input: false,
            buffer: byte_buffer
        }
    }

    // Removal of single and multi line comments
    pub fn remove_comments(&mut self) {
        let mut i = 0;
        //let mut j = 0;
        let sentinel = b'\0';
        let mut buffer: Vec<u8> = Vec::new();
        
        self.buffer.push(b'\0');

        while self.buffer[i] != sentinel {
            let first_letter = self.buffer[i];
            let next_letter = self.buffer[i + 1];
            i = i + 1;

            if first_letter as char == '\n' {
                self.current_line = self.current_line + 1;
            }
            
            if first_letter == b'/' && next_letter == b'*' {
                self.ignore_input = Ignore::MultiLine;
            }

            if first_letter == b'/' && next_letter == b'/' {
                self.ignore_input = Ignore::SingleLine;
            }

            if self.restart_input {
                self.ignore_input = Ignore::No;
                self.restart_input = false;
                continue;
            }

            match self.ignore_input {
                Ignore::MultiLine => {
                    self.restart_input = first_letter == b'*' && next_letter == b'/';
                    continue;
                },
                Ignore::SingleLine => {
                    if first_letter == b'\n' {
                        self.ignore_input = Ignore::No;
                    }
                    continue;
                },
                Ignore::No => ()
            }

            buffer.push(first_letter);
        }
        self.buffer = buffer;
    }

    // Compaction of consecutive white space characters
    pub fn compact_white_space(self) {
        let i = 0;

        while self.buffer[i] != b'\0' {
            println!("{}", self.buffer[i]);
        }
    }

/*
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