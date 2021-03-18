#[derive(Debug)]
pub enum TokenName {
    KEYWORD,
    CONSTANT,
    OPERATOR,
    SYMBOL,
    IDENTIFIER,
}

type SymbolReference = String;
type Token = (TokenName, SymbolReference);


/*
We need to define a symbol table that can store information
about the individual tokens. This is an example of what the
symbol table will need to keep track of. 
This can of course change over time.
*/

enum DataType {
    INT,
    BOOL,
    STRING,
    FLOAT,
    NONE
}

struct SymbolEntry {
    value: String,
    data_type: DataType, 
    line: u32, // what line is this in the source code
    column_index: u32, // where in the column does the word start
    column_offset: u8 // how long is the word
}

pub fn get_next_token() -> Token {

}

// True if `letter` is a whitespace character according to the Schlang syntax.
pub fn is_whitespace(letter: char) -> bool {
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
        | '<'
        | '>'
        | '.'
        | ','
    )
}