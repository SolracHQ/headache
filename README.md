## Headache

Headache is a [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in Rust. It allows you to execute Brainfuck scripts from a file or interactively in real-time interpreter mode.

## Installation

To install Headache, you will need to have Rust and Cargo installed on your system. You can then clone this repository and build the project using Cargo:

```bash
git clone  https://github.com/CarlosEduardoL/headache.git
cd headache
cargo build --release --bin headache --features="build-binary"
```

This will create an executable binary in the `target/release` directory.

## Usage

To execute a Brainfuck script from a file, use the following command:

```bash
./headache path/to/script.bf
```

To run Headache in real-time interpreter mode, use the `-i` flag:

```bash
./headache -i
```

In interpreter mode, you can enter Brainfuck commands one at a time and see their results immediately. To exit interpreter mode, type `exit`.

To execute a string script. use the `-e` flag:

```bash
./headache -e "<literal script>"
```

## Using the Library API

Headache also provides a library API that allows you to execute Brainfuck scripts programmatically from within your own Rust code. Here's an example of how you can use the Headache library API to execute a Brainfuck script:

```rust
use headache::executor::Executor;
use std::io::Cursor;

fn main() {
    // Create a new Executor with input and output streams
    let mut output = Vec::new();
    let mut executor = Executor::new(Cursor::new(b""), Cursor::new(&mut output));

    // Execute a Brainfuck script
    executor.execute("+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.").unwrap();

    // Check the output
    assert_eq!(&output, b"Hello, World!");
}
```

This example creates a new Executor with input and output streams, then executes a Brainfuck script using the execute method. The output of the script is written to the output vector.

You can also customize the input and output streams used by the Executor by passing different types that implement the Read and Write traits when creating a new Executor.

## Contributing

Contributions to Headache are welcome! If you have an idea for a new feature or find a bug, please open an issue or submit a pull request.

## License

Headache is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more information.
