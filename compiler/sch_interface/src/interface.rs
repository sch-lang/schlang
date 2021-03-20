use std::result;

pub type Result<T> = result::Result<T, ErrorReported>;

#[derive(Debug)]
pub struct ErrorReported;

// Useful type to use with `Result<>` indicate that an error has already
// been reported to the user, so no need to continue checking.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FatalErrorMarker;