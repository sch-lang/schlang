#[derive(Debug)]
pub enum TokenName {
    CONSTANT,
    KEYWORD,
    OPERATOR,
    SYMBOL,
    IDENTIFIER,
}

pub type Token = (TokenName, String);

fn is_numeric(word: &str) -> bool {
    for character in word.chars() {
        if !character.is_numeric() {
            return false;
        }
    }

    return true;
}

fn is_boolean(word: &str) -> bool {
    return word == "true" || word == "false";
}

fn is_keyword(word: &str) -> bool {
    let keywords = [
        "strict",
        "use",
        "mut",
        "if",
        "else",
        "return"
    ];

    return keywords.contains(&word);
}

fn is_operator(word: &str) -> bool {
    let operators = [
        "+",
        "-",
        "*",
        "/",
        "=",
        "mod",
        "and",
        "or",
        "is"
    ];

    return operators.contains(&word);
}

fn is_symbol(word: &str) -> bool {
    let symbols = [
        "(",
        ")",
        "[",
        "]",
        "{",
        "}",
        ":",
        ";",
        "<",
        ">",
        ".",
        ","
    ];
    
    return symbols.contains(&word);
}

fn analyze_word(word: &str) -> Token {
    if is_numeric(&word) || is_boolean(&word) {
        return (TokenName::CONSTANT, String::from(word));
    }

    if is_keyword(&word) {
        return (TokenName::KEYWORD, String::from(word));
    }

    if is_operator(&word) {
        return (TokenName::OPERATOR, String::from(word));
    }

    if is_symbol(&word) {
        return (TokenName::SYMBOL, String::from(word));
    }

    return (TokenName::IDENTIFIER, String::from(word));
}

fn is_primitive_operator(letter: char) -> bool {
    let primitive_operators = ['(', ')', '[', ']', '{', '}', ':', ';', '<', '>', '.', ',', '+', '-', '*', '/', '='];

    primitive_operators.contains(&letter)
}

pub fn analyze(text: &String) -> Vec<Token> {
    let letters = text.as_bytes();
    let mut tokens: Vec<Token> = Vec::new();
    let mut word_buffer = text.chars().rev().collect::<String>();
    let mut single_line_comment = false;
    let mut multi_line_comment = false;

    loop {
        let first_letter: char;
        let second_letter: char;

        match word_buffer.pop() {
            Some(letter) => first_letter = letter,
            None => break,
        }

        match word_buffer.pop() {
            Some(letter) => second_letter = letter,
            None => break,
        }

        if first_letter == '/' && second_letter == '*' {
            multi_line_comment = true;
        }

        if first_letter == '/' && second_letter == '/' {
            single_line_comment = true;
        }

        if multi_line_comment && first_letter == '*' && second_letter == '/' {
            multi_line_comment = false;
            continue;
        }

        if single_line_comment && second_letter == '\n' {
            single_line_comment = false;
            continue;
        }

        if multi_line_comment || single_line_comment {
            continue;
        }

        println!("{}{}", first_letter, second_letter);
    }

    word_buffer = String::new();

    for (_, &byte) in letters.iter().enumerate() {
        let letter = byte as char;

        if is_primitive_operator(letter) {
            if word_buffer.len() > 0 {
                tokens.push(analyze_word(&word_buffer));
                word_buffer = String::new();
            }

            tokens.push(analyze_word(&String::from(letter)));
        } else if letter.is_alphanumeric() {
            word_buffer.push(letter);
        } else if (letter == ' ' || letter == '\n') && word_buffer.len() > 0 {
            tokens.push(analyze_word(&word_buffer));
            word_buffer = String::new();
        }
    }

    return tokens;
}