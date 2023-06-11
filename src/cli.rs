use std::{fs, io};
use clap::Parser;
use crate::cli::CLIError::{Cli, IO};
use crate::cli::Mode::{Executor, Interpreted};

#[derive(Parser)]
#[clap(name = "Headache")]
#[clap(version = "0.1.0", author = "CarlosEduardoL")]
struct Headache {
    /// Brainfuck script file
    file: Option<String>,
    /// Run Headache on real-time interpreter mode
    #[clap(short = 'i', long)]
    interpreter: bool,
    /// Execute literal script
    #[clap(short = 'e', long)]
    execute: Option<String>,
}

/// Enum representing the mode in which the Headache program is running.
#[derive(Debug)]
pub enum Mode {
    /// Mode indicating that the program is executing a Brainfuck script from a file.
    Executor(String),
    /// Mode indicating that the program is running in real-time interpreter mode.
    Interpreted,
}

/// Enum representing possible errors that can occur when parsing command line arguments.
#[derive(Debug)]
pub enum CLIError {
    /// Error indicating that an IO error occurred.
    IO(io::Error),
    /// Error indicating that a command line argument error occurred.
    Cli(String),
}

/// Function to determine the mode in which the Headache program should run based on command line arguments.
///
/// # Returns
///
/// * A Result containing either a Mode or a CLIError.
pub fn get_mode() -> Result<Mode, CLIError> {
    let opts: Headache = Headache::parse();

    if let Some(file) = opts.file {
        Ok(Executor(fs::read_to_string(file).map_err(|e| IO(e))?))
    } else if let Some(source) = opts.execute {
        Ok(Executor(source))
    } else if opts.interpreter {
        Ok(Interpreted)
    } else {
        Err(Cli("Error: No file provided and not running in interpreted mode or eval mode".to_string()))
    }
}