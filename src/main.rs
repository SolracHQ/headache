use std::io::{BufRead, BufReader, stdin, stdout, Write};
use std::process::exit;
use crate::cli::{CLIError, get_mode, Mode};
use crate::executor::{execute, execute_with_state, State};
use crate::parser::{parse, ParserError};

mod cli;
mod instruction;
mod parser;
mod input;
mod executor;

/// Main function for the Headache Brainfuck interpreter program.
fn main() {
    // Determine the mode in which the program should run based on command line arguments.
    let mode = match get_mode() {
        Ok(mode) => mode,
        Err(err) => {
            match err {
                CLIError::IO(io) => { eprintln!("Cannot read the script {}", io) }
                CLIError::CLI(err) => { eprintln!("{err}") }
            }
            exit(1)
        }
    };

    // Execute the program based on the determined mode.
    match mode {
        Mode::Executor(source) => {
            // Parse and execute a Brainfuck script from a file.
            match parse(&source) {
                Ok(instructions) => {
                    execute(instructions)
                }
                Err(err) => err.fail()
            }
        }
        Mode::Interpreted => {
            // Run the program in real-time interpreter mode.
            let mut buffer = String::new();
            let mut state = State::new();
            println!("Write exit to finish the interpreter");
            loop {
                if buffer.is_empty() {
                    print!(">")
                } else {
                    print!("==>")
                }
                stdout().flush().unwrap();
                let mut reader = BufReader::new(stdin());
                reader.read_line(&mut buffer).unwrap();
                if buffer.contains("exit") {
                    exit(0)
                }
                match parse(&buffer) {
                    Ok(instructions) => execute_with_state(&instructions, &mut state),
                    Err(_e @ ParserError::IncompleteLoop) => continue,
                    Err(_) => eprintln!("Error: Cannot close ']' without first open '[' it")
                }
                buffer.clear()
            }
        }
    }
}
