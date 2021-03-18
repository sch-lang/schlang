use std::result;

pub type Result<T> = result::Result<T, ErrorReported>;

pub struct ErrorReported;
