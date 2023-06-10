use std::io::{Read, stdin, stdout, Write};
use std::num::Wrapping;
use crate::instruction::Instruction;

/// Constant representing the size of the memory array used by the Brainfuck program.
/// http://brainfuck.org/brainfuck.html
const MEMORY_SIZE: usize = 30_000;

/// Struct representing the state of a Brainfuck program.
pub struct Executor {
    /// Array representing the memory used by the Brainfuck program.
    memory: [Wrapping<u8>; MEMORY_SIZE],
    /// Index representing the current position of the data pointer in the memory array.
    index: usize,
    output: Box<dyn Write>,
    input: Box<dyn Read>,
}

impl Executor {
    /// Function to create a new State with an initialized memory array and index set to 0.
    pub fn new(input: Option<Box<dyn Read>>, output: Option<Box<dyn Write>>) -> Self {
        Self {
            memory: [Wrapping(0u8); MEMORY_SIZE],
            index: 0,
            input: input.unwrap_or(Box::new(stdin())),
            output: output.unwrap_or(Box::new(stdout())),
        }
    }

    /// Function to execute a vector of Brainfuck instructions.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A vector of Instructions to be executed.
    pub fn execute(&mut self, instructions: &[Instruction]) -> std::io::Result<()> {
        for instruction in instructions {
            match instruction {
                Instruction::Move(delta) => {
                    let delta = (MEMORY_SIZE as isize + delta % MEMORY_SIZE as isize) as usize;
                    self.index = (self.index + delta) % MEMORY_SIZE;
                }
                Instruction::Add(n) => { self.memory[self.index] += *n }
                Instruction::Write => {
                    self.output.write_all(&[self.memory[self.index].0])?;
                }
                Instruction::Read => {
                    let mut buffer = [0u8];
                    self.input.read_exact(&mut buffer)?;
                    self.memory[self.index] = Wrapping(buffer[0]);
                }
                Instruction::Loop(instructions) => {
                    while self.memory[self.index].0 != 0 {
                        self.execute(&instructions)?;
                    }
                }
                Instruction::Clear => self.memory[self.index] = Wrapping(0),
                Instruction::AddTo { offset } => {
                    let delta = (MEMORY_SIZE as isize + offset % MEMORY_SIZE as isize) as usize;
                    let to = (self.index + delta) % MEMORY_SIZE;

                    self.memory[to] += self.memory[self.index];
                    self.memory[self.index] = Wrapping(0);
                }
            }
        }
        Ok(())
    }
}