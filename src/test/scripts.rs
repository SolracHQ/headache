/// From https://en.wikipedia.org/wiki/Brainfuck
pub const HELLO_WORLD: &str =
    "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
/// From https://en.wikipedia.org/wiki/Brainfuck
pub const SHORTER_HELLO_WORLD: &str = "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.";
/// From https://en.wikipedia.org/wiki/Brainfuck
pub const COMMENTED_HELLO_WORLD: &str = include_str!("scripts/hello_world.bf");
/// From https://en.wikipedia.org/wiki/Brainfuck
pub const ADD: &str = include_str!("scripts/add.bf");
/// From Erik Bosman
pub const MANDELBROT: &str = include_str!("scripts/mandelbrot.bf");