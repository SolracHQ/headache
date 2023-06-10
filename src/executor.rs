use std::io::{stdout, Write};
use std::num::Wrapping;
use crate::input::getchar;
use crate::instruction::Instruction;

/// Constant representing the size of the memory array used by the Brainfuck program.
const MEMORY_SIZE: usize = 1024 * 4;

/// Struct representing the state of a Brainfuck program.
pub struct State {
    /// Array representing the memory used by the Brainfuck program.
    memory: [Wrapping<u8>; MEMORY_SIZE],
    /// Index representing the current position of the data pointer in the memory array.
    index: usize
}

impl State {
    /// Function to create a new State with an initialized memory array and index set to 0.
    pub fn new() -> Self {
        Self {
            memory: [Wrapping(0u8); MEMORY_SIZE],
            index: 0
        }
    }
}

/// Function to execute a vector of Brainfuck instructions.
///
/// # Arguments
///
/// * `instructions` - A vector of Instructions to be executed.
pub fn execute(instructions: Vec<Instruction>) {
    execute_with_state(&instructions, &mut State::new())
}

/// Function to execute a slice of Brainfuck instructions with a given State.
///
/// # Arguments
///
/// * `instructions` - A slice of Instructions to be executed.
/// * `state` - A mutable reference to a State representing the current state of the Brainfuck program.
pub fn execute_with_state(instructions: &[Instruction], state: &mut State) {
    for instruction in instructions {
        match instruction {
            Instruction::Increase => state.index = (state.index + 1) % MEMORY_SIZE,
            Instruction::Decrease => state.index = (state.index - 1) % MEMORY_SIZE,
            Instruction::Increment => state.memory[state.index] += 1,
            Instruction::Decrement => state.memory[state.index] -= 1,
            Instruction::Write => {
                print!("{}", state.memory[state.index].0 as char);
                stdout().flush().unwrap();
            }
            Instruction::Read => {
                state.memory[state.index] = Wrapping(getchar() as u8);
            }
            Instruction::Loop(instructions) => {
                while state.memory[state.index].0 != 0 {
                    execute_with_state(&instructions, state)
                }
            }
        }
    }
}
