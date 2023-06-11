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

## Contributing

Contributions to Headache are welcome! If you have an idea for a new feature or find a bug, please open an issue or submit a pull request.

## License

Headache is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more information.
