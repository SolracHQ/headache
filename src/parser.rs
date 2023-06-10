use std::process::exit;
use ParserError::IncompleteLoop;
use crate::instruction::Instruction;
use crate::instruction::Instruction::AddTo;
use crate::parser::ParserError::UnexpectedToken;

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
    let mut contexts = vec![vec![]];

    for char in source.chars() {
        let instruction = match char {
            '>' => {
                if let Some(Instruction::Move(n)) = contexts.last_mut().unwrap().last_mut() {
                    *n += 1;
                    continue;
                }
                Instruction::Move(1)
            }
            '<' => {
                if let Some(Instruction::Move(n)) = contexts.last_mut().unwrap().last_mut() {
                    *n -= 1;
                    continue;
                }
                Instruction::Move(-1)
            }
            '+' => {
                if let Some(Instruction::Add(n)) = contexts.last_mut().unwrap().last_mut() {
                    *n += 1;
                    continue;
                }
                Instruction::Add(1)
            }
            '-' => {
                if let Some(Instruction::Add(n)) = contexts.last_mut().unwrap().last_mut() {
                    *n -= 1;
                    continue;
                }
                Instruction::Add(1u8.wrapping_neg())
            }
            '.' => Instruction::Write,
            ',' => Instruction::Read,
            '[' => {
                contexts.push(vec![]);
                continue;
            }
            ']' => {
                let Some(instructions) = contexts.pop() else {
                    return Err(UnexpectedToken);
                };
                let Some(current_context) = contexts.last_mut() else {
                    return Err(UnexpectedToken);
                };
                match instructions[..] {
                    [Instruction::Add(n)] if n & 1 == 1 => {
                        current_context.push(Instruction::Clear);
                        continue;
                    }
                    [Instruction::Add(255), Instruction::Move(x), Instruction::Add(1), Instruction::Move(y)]
                    if x == -y => {
                        current_context.push(AddTo { offset: x })
                    }
                    _ => {}
                }
                current_context.push(Instruction::Loop(instructions));
                continue;
            }
            _ => continue
        };
        let Some(current_context) = contexts.last_mut() else {
            return Err(IncompleteLoop);
        };
        current_context.push(instruction);
    }
    if contexts.len() != 1 {
        return Err(IncompleteLoop);
    }
    Ok(contexts.pop().unwrap())
}
