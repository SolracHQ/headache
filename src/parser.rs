use crate::error::ParserError;
use crate::error::ParserError::{IncompleteLoop, UnexpectedToken};
use crate::instruction::Instruction;
use crate::instruction::Instruction::AddTo;

/// Function to parse a Brainfuck source code string into a vector of Instructions.
///
/// # Arguments
///
/// * `source` - A string slice containing the Brainfuck source code.
///
/// # Returns
///
/// * A Result containing either a vector of Instructions or a `ParserError`.
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
                        current_context.push(AddTo { offset: x });
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
