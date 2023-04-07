# Crab 🦀 (Rust powered cat copy)


Crab is a simple project that recreates the cat command in Rust. It uses exactly the same flags and arguments as cat, making it easy to use as a drop-in replacement.

## Building
To build Crab, you'll need to have Rust installed on your machine. Once you have Rust installed, you can build Crab using Cargo:
```bash
git clone https://github.com/your_username/crab.git
cd crab
cargo build --release
```
The binary will be located at target/release/crab. You can then move the binary to a location in your $PATH to make it globally accessible.

## Usage
Here are some examples of how to use Crab:
```bash
# Concatenate two files and display line numbers
crab -n file1.txt file2.txt

# Display a file with line numbers and show tabs as "^I"
crab -nT file.txt

# Display a file with non-printing characters shown using "^" and "M-"
crab -v file.txt
```

## Arguments
Here are the arguments that Crab supports:

- FILES *Positional* : Files to be concatenated. If - is used instead of a filename, Crab will read from standard input.
- -n, --number: Add a number on the line start.
- -b, --number-nonblank: Number nonempty output lines, overrides -n.
- -A, --show-all: Equivalent to -vET.
- -v: Use ^ and M- notation to show non-printing characters (except for LFD and TAB).
- -E, --show-ends: Show end of lines with $.
- -T, --show-tabs: Display TAB characters as ^I.
- --ev: Equivalent to -vE.
- -t: Equivalent to -vT.
- --squeeze-blank: Suppress repeated empty output lines.

## License
This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE) file for more information.