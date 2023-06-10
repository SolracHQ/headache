/// Enum representing the different instructions that can be used in a Brainfuck program.
#[derive(Debug)]
pub enum Instruction {
    /// Instruction to increase the data pointer (to point to the next cell to the right).
    Increase,
    /// Instruction to decrease the data pointer (to point to the next cell to the left).
    Decrease,
    /// Instruction to increment (increase by one) the byte at the data pointer.
    Increment,
    /// Instruction to decrement (decrease by one) the byte at the data pointer.
    Decrement,
    /// Instruction to output the byte at the data pointer as an ASCII character.
    Write,
    /// Instruction to accept one byte of input and store its value at the data pointer.
    Read,
    /// Instruction to execute a loop of instructions while the value at the data pointer is non-zero.
    Loop(Vec<Instruction>)
}