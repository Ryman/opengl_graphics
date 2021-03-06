//! Errors

use std::fmt;

/// An enum to represent various possible run-time errors that may occur.
#[derive(Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// An error happened with the FreeType library.
    FreetypeError(::freetype::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
