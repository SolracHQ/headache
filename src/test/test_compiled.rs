use crate::compiler::compile;
use crate::test::MANDELBROT_RESULT;
use crate::test::scripts::{
    ADD, COMMENTED_HELLO_WORLD, HELLO_WORLD, MANDELBROT, SHORTER_HELLO_WORLD,
};
use std::io::stdin;

fn execute_with_output(program: &str) -> String {
    let mut result = Vec::new();
    let mut stdin = stdin();
    let executable = compile(program, &mut stdin, &mut result).unwrap();
    executable.run().unwrap();
    std::str::from_utf8(&result).unwrap().to_string()
}

#[test]
fn test_hello_world_1() {
    let output = execute_with_output(HELLO_WORLD);
    assert_eq!(&output, "Hello World!\n");
}

#[test]
fn test_hello_world_2() {
    let output = execute_with_output(SHORTER_HELLO_WORLD);
    assert_eq!(&output, "Hello, World!");
}

#[test]
fn test_hello_world_3() {
    let output = execute_with_output(COMMENTED_HELLO_WORLD);
    assert_eq!(&output, "Hello World!\n");
}

#[test]
fn test_sum() {
    let output = execute_with_output(ADD);
    assert_eq!(&output, "7");
}

#[test]
fn test_mandelbrot() {
    let output = execute_with_output(MANDELBROT);
    assert_eq!(&output, MANDELBROT_RESULT);
}