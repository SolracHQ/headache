#[cfg(test)]
mod test;

/// Constant representing the size of the memory array used by the Brainfuck program.
/// [DOC](http://brainfuck.org/brainfuck.html)
pub const MEMORY_SIZE: usize = 30_000;

pub mod error;
pub mod executor;
mod instruction;
mod parser;
#[cfg(target_arch="x86_64")]
pub mod compiler;