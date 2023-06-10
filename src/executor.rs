use std::io::{stdout, Write};
use std::num::Wrapping;
use crate::input::getchar;
use crate::instruction::Instruction;

/// Constant representing the size of the memory array used by the Brainfuck program.
/// http://brainfuck.org/brainfuck.html
const MEMORY_SIZE: usize = 30_000;

/// Struct representing the state of a Brainfuck program.
#[derive(Debug)]
pub struct State {
    /// Array representing the memory used by the Brainfuck program.
    memory: [Wrapping<u8>; MEMORY_SIZE],
    /// Index representing the current position of the data pointer in the memory array.
    index: usize,
}

impl State {
    /// Function to create a new State with an initialized memory array and index set to 0.
    pub fn new() -> Self {
        Self {
            memory: [Wrapping(0u8); MEMORY_SIZE],
            index: 0,
        }
    }
}

/// Function to execute a vector of Brainfuck instructions.
///
/// # Arguments
///
/// * `instructions` - A vector of Instructions to be executed.
pub fn execute(instructions: Vec<Instruction>) {
    execute_with_state(&instructions, &mut State::new());
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
            Instruction::Move(delta) => {
                let delta = (MEMORY_SIZE as isize + delta % MEMORY_SIZE as isize) as usize;
                state.index = (state.index + delta) % MEMORY_SIZE;
            }
            Instruction::Add(n) => { state.memory[state.index] += *n }
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
            Instruction::Clear => state.memory[state.index] = Wrapping(0),
            Instruction::AddTo { offset } => {
                let delta = (MEMORY_SIZE as isize + offset % MEMORY_SIZE as isize) as usize;
                let to = (state.index + delta) % MEMORY_SIZE;

                state.memory[to] += state.memory[state.index];
                state.memory[state.index] = Wrapping(0);
            }
        }
    }
}
