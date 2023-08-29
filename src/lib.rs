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

pub fn test() -> u8 {
    let mut arr = [0u8; MEMORY_SIZE];
    code(arr.as_mut_ptr(), 0, unsafe {offset()});
    arr[0]
}

#[inline(never)]
extern "sysv64" fn code(mem: *mut u8, index: usize, offset: isize) {
    unsafe {
        let to = ((index as isize + offset) % MEMORY_SIZE as isize + MEMORY_SIZE as isize) % MEMORY_SIZE as isize;
        *mem.add(to as usize) += *mem.add(index);
        *mem.add(index) = 0;
    }
}

extern "C" {
    fn offset() -> isize;
}