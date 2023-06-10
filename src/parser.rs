use std::process::exit;
use ParserError::IncompleteLoop;
use crate::instruction::Instruction;
use crate::parser::ParserError::UnexpectedToken;

/// Enum representing possible errors that can occur during parsing.
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
            IncompleteLoop => {
                eprintln!("All the '[' instructions must be closed with a ']' instruction");
            }
            UnexpectedToken => {
                eprintln!("Cannot close ']' without first open '[' it");
            }
        }
        exit(1)
    }
}

/// Function to parse a Brainfuck source code string into a vector of Instructions.
///
/// # Arguments
///
/// * `source` - A string slice containing the Brainfuck source code.
///
/// # Returns
///
/// * A Result containing either a vector of Instructions or a ParserError.
pub fn parse(source: &str) -> Result<Vec<Instruction>, ParserError> {
    let mut result = vec![];
    let mut loops = vec![];

    for char in source.chars() {
        let instruction = match char {
            '>' => Instruction::Increase,
            '<' => Instruction::Decrease,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '.' => Instruction::Write,
            ',' => Instruction::Read,
            '[' => {
                loops.push(vec![]);
                continue;
            }
            ']' => {
                let Some(instructions) = loops.pop() else {
                    return Err(UnexpectedToken);
                };
                let Some(current_loop) = loops.last_mut() else {
                    result.push(Instruction::Loop(instructions));
                    continue;
                };
                current_loop.push(Instruction::Loop(instructions));
                continue;
            }
            _ => continue
        };
        let Some(current_loop) = loops.last_mut() else {
            result.push(instruction);
            continue;
        };
        current_loop.push(instruction);
    }
    if !loops.is_empty() {
        return Err(IncompleteLoop);
    }
    Ok(result)
}
