use std::fmt;
use std::error::Error;

/// Custom error enum for the `expected` function.
///
/// This type complies with the `std::error::Error` trait, which means it is compatible
/// with Rust's standard error handling mechanisms.
#[derive(Debug)]
pub enum SymbolMappingError {
    SymbolNotFoundInCodes(char),
    ExtraSymbolInCodes(char),
    UnknownSymbolInString,
    // Add other types of errors as needed
}

// Implement the Display trait for ExpectedError
impl fmt::Display for SymbolMappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolMappingError::SymbolNotFoundInCodes(symbol) => {
                write!(f, "Symbol {} found in frequencies but not in codes.", symbol)
            },
            SymbolMappingError::ExtraSymbolInCodes(symbol) => {
                write!(f, "Extra symbol {} found in codes but not in frequencies.", symbol)
            },
            SymbolMappingError::UnknownSymbolInString => {
                write!(f, "Unknown symbol found in the input string.")
            },
        }
    }
}

// Implement the Error trait for ExpectedError
impl Error for SymbolMappingError {}
