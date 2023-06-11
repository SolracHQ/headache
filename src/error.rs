use std::process::exit;

#[derive(Debug)]
pub enum Error {
    ParseError(ParserError),
    RuntimeError(std::io::Error),
}

/// Enum representing possible errors that can occur during parsing.
#[derive(Debug)]
pub enum ParserError {
    /// Error indicating that a loop was not closed properly.
    IncompleteLoop,
    /// Error indicating that an unexpected token was encountered.
    UnexpectedToken,
}

impl ParserError {
    /// Function to handle parser errors and exit the program.
    pub fn fail(self) -> ! {
        match self {
            ParserError::IncompleteLoop => {
                eprintln!("All the '[' instructions must be closed with a ']' instruction");
            }
            ParserError::UnexpectedToken => {
                eprintln!("Cannot close ']' without first open '[' it");
            }
        }
        exit(1)
    }
}