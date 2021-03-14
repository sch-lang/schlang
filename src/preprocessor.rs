/**
 * The preprocessor function will read all the `use` statements
 * in the source language and concatenate all dependencies in the
 * correct order, and then transform the text into a byte buffer.
 *
 * If the preprocessor finds an unknown dependency, it will mark it
 * as an unrecoverable error and move on. The Lexical Analyzer will
 * then produce the error later with the appropriate line number.
*/
pub fn process(text: &String) -> [u8] {
    // TODO: recursively prepend all dependencies found in use statements

    return text.as_bytes();
}