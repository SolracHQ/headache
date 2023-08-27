/// Enum representing the different instructions that can be used in a Brainfuck program.
#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum Instruction {
    /// Instruction to increase the data pointer (to point to the isize neighbor).
    Move(isize),
    /// Instruction to increment (increase by isize) the byte at the data pointer.
    Add(u8),
    /// Instruction to output the byte at the data pointer as an ASCII character.
    Write,
    /// Instruction to accept one byte of input and store its value at the data pointer.
    Read,
    /// Instruction to execute a loop of instructions while the value at the data pointer is non-zero.
    Loop(Vec<Instruction>),
    /// Set the current data to 0
    Clear,
    /// Add current data to value on pointer + offset and set current data to 0
    MoveTo{ offset: isize },
}