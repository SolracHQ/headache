use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmLabelApi};

use crate::MEMORY_SIZE;
use crate::{error::Error, instruction::Instruction, parser::parse};
use std::io::{Read, Write};
use std::marker::PhantomData;

/// Struct representing a compiled Brainfuck program.
///
/// The `Executable` struct contains an `Assembler` object from the `dynasmrt` crate,
/// which holds the generated machine code for the Brainfuck program.
pub struct Executable<'a> {
    code: dynasmrt::Assembler<X64Relocation>,
    _in_out_life_time: PhantomData<&'a ()>,
}

impl<'a> Executable<'a> {
    /// Function to create a new `Executable` object.
    ///
    /// # Arguments
    ///
    /// * `code` - An `Assembler` object from the `dynasmrt` crate containing the generated
    ///            machine code for the Brainfuck program.
    fn new(code: dynasmrt::Assembler<X64Relocation>) -> Self {
        Executable {
            code,
            _in_out_life_time: PhantomData {},
        }
    }

    /// Function to run the compiled Brainfuck program.
    ///
    /// This function finalizes the machine code and executes it, passing in a memory array
    /// to be used by the Brainfuck program.
    ///
    /// # Errors
    ///
    /// This function returns an error if any of the following conditions are met:
    ///
    /// * An I/O error occurs while reading from the input stream or writing to the output stream.
    ///   In this case, a `RuntimeError` is returned containing the underlying I/O error.
    pub fn run(self) -> Result<(), Error> {
        let buffer = self.code.finalize().unwrap();
        let mut memory = [0u8; MEMORY_SIZE];

        unsafe {
            let exe: unsafe extern "sysv64" fn(*mut u8) -> *mut std::io::Error =
                std::mem::transmute(buffer.as_ptr());
            let err = exe(memory.as_mut_ptr());

            if !err.is_null() {
                return Err(Error::RuntimeError(*Box::from_raw(err)));
            }
        }
        Ok(())
    }
}

/// Function to compile a Brainfuck program into an `Executable` object.
///
/// This function takes a string containing Brainfuck source code, along with input and output
/// streams, and compiles it into an `Executable` object. It does this by first parsing the source
/// code into a vector of `Instruction` values using the `parse` function from the `parser` module.
/// Then, it generates machine code for each instruction using the `dynasm!` macro from the
/// `dynasmrt` crate.
///
/// # Arguments
///
/// * `source` - A string containing Brainfuck source code to be compiled.
/// * `input` - An input stream to be used for reading data into the Brainfuck program.
/// * `out` - An output stream to be used for writing data from the Brainfuck program.
///
/// # Errors
///
/// This function returns an error if any of the following conditions are met:
///
/// * The given Brainfuck code cannot be parsed successfully. In this case, a `ParseError`
///   is returned containing the specific parsing error that occurred.
pub fn compile<'a, Input: Read, Output: Write>(
    source: &str,
    input: &'a mut Input,
    out: &'a mut Output,
) -> Result<Executable<'a>, Error> {
    let instructions = parse(source).map_err(Error::ParseError)?;
    let mut code: dynasmrt::Assembler<X64Relocation> =
        dynasmrt::x64::Assembler::new().map_err(Error::CompileError)?;

    dynasm! { code
        ; .arch x64
        ; push rbp
        ; mov rbp, rsp
        ; push r12 // pointer to memory
        ; push r13 // offset from r12
        ; mov r12, rdi
        ; xor r13, r13
    };

    compile_segment(&instructions, &mut code, input, out);

    dynasm! { code
        ; .arch x64
        ; xor rax, rax // clear return register if not early return
        ; ->exit:
        ; pop r13
        ; pop r12
        ; pop rbp
        ; ret
    }
    Ok(Executable::new(code))
}

/// Function to generate machine code for a segment of Brainfuck instructions.
///
/// This function takes a slice of `Instruction` values and generates machine code for each instruction
/// using the `dynasm!` macro from the `dynasmrt` crate. It is a helper function used by the `compile`
/// function.
///
/// # Arguments
///
/// * `instructions` - A slice of `Instruction` values to be compiled.
/// * `code` - An `Assembler` object from the `dynasmrt` crate to which the generated machine code
///            will be added.
/// * `input` - An input stream to be used for reading data into the Brainfuck program.
/// * `out` - An output stream to be used for writing data from the Brainfuck program.
fn compile_segment<'a, Input: Read, Output: Write>(
    instructions: &[Instruction],
    code: &mut dynasmrt::Assembler<X64Relocation>,
    input: &'a Input,
    out: &'a Output,
) {
    for instruction in instructions {
        match instruction {
            Instruction::Move(n) => {
                dynasm! { code
                    ; .arch x64
                    ; mov     rcx, QWORD *n as _
                    ; mov     rdx, QWORD 5037190915060954895
                    ; mov     rax, rcx
                    ; imul    rdx
                    ; mov     rax, rdx
                    ; shr     rax, 63
                    ; sar     rdx, 13
                    ; add     rdx, rax
                    ; imul    rax, rdx, 30000
                    ; sub     rcx, rax
                    ; mov     rax, r13
                    ; add     rcx, rax
                    ; add     rcx, 30000
                    ; mov     rax, rcx
                    ; shr     rax, 4
                    ; mov     rdx, QWORD 314824432191309681
                    ; mul     rdx
                    ; shr     rdx, 5
                    ; imul    rax, rdx, 30000
                    ; sub     rcx, rax
                    ; mov     r13, rcx
                }
            }
            Instruction::Add(n) => {
                dynasm! { code
                    ; .arch x64
                    ; add     BYTE [r12 + r13], *n as i8
                }
            }
            Instruction::Write => {
                dynasm! { code
                    ; .arch x64
                    ; lea     rdi, [r12 + r13]
                    ; mov     rsi, QWORD unsafe { std::mem::transmute::<&'a Output, _>(out) }
                    ; mov     rax, QWORD write::<Output> as _
                    ; call    rax
                    ; cmp     rax, 0
                    ; jne     ->exit
                }
            }
            Instruction::Read => {
                dynasm! { code
                    ; .arch x64
                    ; lea     rdi, [r12 + r13]
                    ; mov     rsi, QWORD unsafe { std::mem::transmute::<&'a Input, _>(input) }
                    ; mov     rax, QWORD read::<Input> as _
                    ; call    rax
                    ; cmp     rax, 0
                    ; jne     ->exit
                }
            }
            Instruction::Loop(loop_segment) => {
                let loop_label = code.new_dynamic_label();
                let end_label = code.new_dynamic_label();
                dynasm! { code
                    ; .arch x64
                    ; =>loop_label
                    ; cmp     BYTE [r12 + r13], 0
                    ; je      =>end_label
                }
                compile_segment(&loop_segment, code, input, out);
                dynasm! { code
                    ; .arch x64
                    ; cmp     BYTE [r12+r13], 0
                    ; jne     =>loop_label
                    ; =>end_label
                }
            }
            Instruction::Clear => {
                dynasm! { code
                    ; .arch x64
                    ; mov     BYTE [r12+r13], 0
                }
            }
            Instruction::MoveTo { offset } => {
                compile_segment(&[Instruction::Loop(vec![Instruction::Add(255), Instruction::Move(*offset), Instruction::Add(1), Instruction::Move(-offset)])], code, input, out)
            }
        }
    }
}

/// Function to read a single byte from an input stream.
///
/// This function is an extern "sysv64" function that is called by the generated machine code to read a single byte from the input stream and store it in the given memory location. It returns a null pointer if the read is successful or a pointer to an `std::io::Error` object if an error occurs.
///
/// # Arguments
///
/// * `ptr` - A pointer to the memory location where the read byte should be stored.
/// * `input` - A pointer to the input stream from which to read the byte.
///
/// # Safety
///
/// This function is unsafe because it takes raw pointers as arguments and dereferences them. The caller must ensure that the pointers are valid and that it is safe to dereference them.
extern "sysv64" fn read<Input: Read>(ptr: &mut [u8; 1], input: &mut Input) -> *mut std::io::Error {
    match input.read_exact(ptr) {
        Ok(_) => std::ptr::null_mut(),
        Err(err) => Box::into_raw(Box::new(err)),
    }
}

/// Function to write a single byte to an output stream.
///
/// This function is an extern "sysv64" function that is called by the generated machine code to write a single byte from the given memory location to the output stream. It returns a null pointer if the write is successful or a pointer to an `std::io::Error` object if an error occurs.
///
/// # Arguments
///
/// * `ptr` - A pointer to the memory location containing the byte to be written.
/// * `out` - A pointer to the output stream to which the byte should be written.
///
/// # Safety
///
/// This function is unsafe because it takes raw pointers as arguments and dereferences them. The caller must ensure that the pointers are valid and that it is safe to dereference them.
extern "sysv64" fn write<Output: Write>(
    ptr: &mut [u8; 1],
    out: &mut Output,
) -> *mut std::io::Error {
    match out.write_all(ptr) {
        Ok(_) => std::ptr::null_mut(),
        Err(err) => Box::into_raw(Box::new(err)),
    }
}

