use std::io::{Read, stdin, Stdin, stdout, Stdout, Write};
use std::num::Wrapping;
use crate::error::Error;
use crate::error::Error::RuntimeError;
use crate::instruction::Instruction;
use crate::parser::parse;

/// Constant representing the size of the memory array used by the Brainfuck program.
/// http://brainfuck.org/brainfuck.html
const MEMORY_SIZE: usize = 30_000;

/// Struct representing the state of a Brainfuck program.
///
/// The `Executor` struct contains the memory array used by the Brainfuck program,
/// as well as the current position of the data pointer in the memory array. It also
/// contains input and output streams for reading and writing data.
pub struct Executor<Input: Read, Output: Write> {
    /// Array representing the memory used by the Brainfuck program.
    ///
    /// This is an array of `Wrapping<u8>` values, where each value represents a single
    /// memory cell in the Brainfuck program. The size of the array is determined by the
    /// `MEMORY_SIZE` constant.
    memory: [Wrapping<u8>; MEMORY_SIZE],
    /// Index representing the current position of the data pointer in the memory array.
    ///
    /// This value is used to keep track of which memory cell is currently being accessed
    /// by the Brainfuck program. It is updated whenever a move instruction is executed.
    index: usize,
    /// Input stream used for reading data into the Brainfuck program.
    ///
    /// This can be any type that implements the `Read` trait. If no input stream is provided
    /// when creating a new `Executor`, `stdin` is used by default.
    input: Input,
    /// Output stream used for writing data from the Brainfuck program.
    ///
    /// This can be any type that implements the `Write` trait. If no output stream is provided
    /// when creating a new `Executor`, `stdout` is used by default.
    output: Output,
}

impl <Input: Read, Output: Write> Executor<Input, Output> {
    /// Function to create a new State with an initialized memory array and index set to 0.
    ///
    /// # Arguments
    ///
    /// * `input` - An input stream to be used for reading data into the Brainfuck program.
    /// * `output` - An output stream to be used for writing data from the Brainfuck program.
    pub fn new(input: Input, output: Output) -> Self {
        Self {
            memory: [Wrapping(0u8); MEMORY_SIZE],
            index: 0,
            input,
            output,
        }
    }

    /// Function to execute a string of Brainfuck code.
    ///
    /// This function takes a string containing Brainfuck code and executes it. The code is first
    /// parsed into a vector of `Instruction` values using the `parse` function from the `parser`
    /// module. The resulting instructions are then executed in order using the `_execute`
    /// function.
    ///
    /// # Arguments
    ///
    /// * `code` - A string containing Brainfuck code to be executed.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::Cursor;
    /// use headache::executor::Executor;
    ///
    /// let mut output = Vec::new();
    /// let mut executor = Executor::new(Cursor::new(b""), Cursor::new(&mut output));
    ///
    /// executor.execute("+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.").unwrap();
    ///
    /// assert_eq!(&output, b"Hello, World!");
    /// ```
    pub fn execute(&mut self, code: &str) -> Result<(), Error> {
        let instructions = match parse(code) {
            Ok(instructions) => instructions,
            Err(err) => {
                return Err(Error::ParseError(err))
            }
        };
        self._execute(&instructions)
    }

    /// Function to execute a vector of Brainfuck instructions.
    ///
    /// This function takes a slice of `Instruction` values and executes them in order. The behavior
    /// of each instruction is determined by its variant:
    ///
    /// * `Move(delta)` - Moves the data pointer by `delta` positions. If `delta` is positive, the
    ///                   data pointer is moved to the right; if it is negative, it is moved to the left.
    /// * `Add(n)` - Adds `n` to the value of the current memory cell.
    /// * `Write` - Writes the value of the current memory cell to the output stream.
    /// * `Read` - Reads a value from the input stream and stores it in the current memory cell.
    /// * `Loop(instructions)` - Executes a loop. The loop body consists of the given `instructions`,
    ///                          which are executed repeatedly until the value of the current memory cell
    ///                          becomes 0.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A vector of Instructions to be executed.
    fn _execute(&mut self, instructions: &[Instruction]) -> Result<(), Error> {
        for instruction in instructions {
            match instruction {
                Instruction::Move(delta) => {
                    let delta = (MEMORY_SIZE as isize + delta % MEMORY_SIZE as isize) as usize;
                    self.index = (self.index + delta) % MEMORY_SIZE;
                }
                Instruction::Add(n) => { self.memory[self.index] += *n }
                Instruction::Write => {
                    self.output.write_all(&[self.memory[self.index].0]).map_err(RuntimeError)?;
                    self.output.flush().map_err(RuntimeError)?;
                }
                Instruction::Read => {
                    let mut buffer = [0u8];
                    self.input.read_exact(&mut buffer).map_err(RuntimeError)?;
                    self.memory[self.index] = Wrapping(buffer[0]);
                }
                Instruction::Loop(instructions) => {
                    while self.memory[self.index].0 != 0 {
                        self._execute(instructions)?;
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

impl Default for Executor<Stdin, Stdout> {
    /// Function to create a new State with an initialized memory array and index set to 0.
    ///
    /// This function creates a new `Executor` with `stdin` as the input stream and `stdout`
    /// as the output stream.
    fn default() -> Self {
        Self {
            memory: [Wrapping(0u8); MEMORY_SIZE],
            index: 0,
            input: stdin(),
            output: stdout(),
        }
    }
}
