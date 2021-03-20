#[derive(Clone, Debug)]
pub enum TokenName {
    KEYWORD,
    CONSTANT,
    OPERATOR,
    SYMBOL,
    IDENTIFIER,
}

#[derive(Clone, Debug)]
pub struct Token(TokenName, usize);

impl Copy for Token {}
impl Copy for TokenName {}

// True if `letter` is a whitespace character according to the Schlang syntax.
fn is_whitespace(letter: char) -> bool {
    matches!(letter, 
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space
    )
}

fn is_binary_operator(letter: char) -> bool {
    matches!(letter,
        '='
        | '+'
        | '-'
        | '*'
        | '/'
        | '<'
        | '>'
        | '^'
    )
}

fn is_symbol(letter: char) -> bool {
    matches!(letter,
        '('
        | ')'
        | '['
        | ']'
        | '{'
        | '}'
        | ':'
        | ';'
        | '.'
        | ','
    )
}

fn is_keyword(word: &String) -> bool {
    matches!(word.as_str(),
        "if"
        | "else"
        | "use"
        | "return"
        | "strict"
        | "class"
        | "public"
        | "local"
        | "mut"
        | "or"
        | "is"
        | "and"
        | "more"
        | "than"
        | "less"
        | "mod"
    )
}

fn is_constant(word: &String) -> bool {
    let is_numeric: bool = word.chars().all(char::is_numeric);
    let is_bool = matches!(word.as_str(), "false" | "true");

    return is_numeric || is_bool;
}

fn remove_comments(mut buffer: Vec<char>) -> Vec<char> {
    let mut block_comment = false;
    let mut single_comment = false;
    let mut new_buffer: Vec<char> = Vec::new();
    let mut i = 0;
    let sentinel = '\0';

    buffer.push(sentinel);

    loop {
        if i > 0 {
            i += 1;
        }

        if buffer[i] == sentinel {
            break;
        }

        if single_comment && buffer[i] == '\n' {
            single_comment = false;
        } else if block_comment && buffer[i] == '*' && buffer[i + 1] == '/' {
            block_comment = false;
            i += 1;
        } else if single_comment || block_comment {
            continue;
        } else if buffer[i] == '/' && buffer[i + 1] == '/' {
            single_comment = true;
            i += 1;
        } else if buffer[i] == '/' && buffer[i + 1] == '*' {
            block_comment = true;
            i += 1;
        } else {
            new_buffer.push(buffer[i]);
        }
    }

    return new_buffer;
}

fn compact_white_spaces(mut buffer: Vec<char>) -> Vec<char> {
    let mut new_buffer: Vec<char> = Vec::new();
    let mut i = 0;
    let sentinel = '\0';
    buffer.push(sentinel);

    while buffer[i] != sentinel {
        let first_letter = buffer[i];
        let next_letter = buffer[i + 1];

        if is_whitespace(first_letter) && is_whitespace(next_letter) {
            i += 1;
            continue;
        }

        i += 1;
        new_buffer.push(first_letter);
    }

    // remove leading whitespace if present
    if is_whitespace(new_buffer[0]) {
        new_buffer.remove(0);
    }

    return new_buffer;
}

pub struct LexicalAnalyzer {
    buffer: Vec<char>,
    token_stream: Vec<Token>,
    cursor: usize
}

impl LexicalAnalyzer {
    pub fn new(buffer: Vec<char>) -> Self {
        let mut lexical_analyzer = Self {
            token_stream: Vec::new(),
            cursor: 0,
            buffer
        };
        
        lexical_analyzer.run_analysis();
        
        return lexical_analyzer;
    }

    pub fn get_next_token(&mut self) -> Token {
        let next_token = &self.token_stream[self.cursor];
        self.cursor += 1;
        
        return *next_token;
    }

    fn run_analysis(&mut self) {
        let mut i = 0;
        //let mut local_token_stream = &self.token_stream;
        let mut local_buffer = self.buffer.to_vec();
        let mut word_buffer = String::new();
        let sentinel = '\0';
    
        local_buffer = remove_comments(local_buffer);
        local_buffer = compact_white_spaces(local_buffer);
    
        local_buffer.push(sentinel);
    
        while local_buffer[i] != sentinel {
            let this_char = local_buffer[i];
            let next_char = local_buffer[i + 1];
    
            if this_char.is_alphanumeric() {
                word_buffer.push(this_char);
            }
    
            if is_binary_operator(this_char) {
                &self.token_stream.push(Token(TokenName::OPERATOR, i));
                word_buffer = String::new();
                i += 1;
                continue;
            }
            
            if is_symbol(this_char) {
                &self.token_stream.push(Token(TokenName::SYMBOL, i));
                word_buffer = String::new();
                i += 1;
                continue;
            }
    
            if word_buffer.len() > 0 && (
                is_whitespace(next_char) 
                || is_symbol(next_char) 
                || is_binary_operator(next_char)) {
                if is_keyword(&word_buffer) {
                    &self.token_stream.push(Token(TokenName::KEYWORD, i));
                } else if is_constant(&word_buffer) {
                    &self.token_stream.push(Token(TokenName::CONSTANT, i));
                } else {
                    &self.token_stream.push(Token(TokenName::IDENTIFIER, i));
                }
    
                word_buffer = String::new();
                i += 1;
                continue;
            }
    
            i += 1;
        }
    }
}