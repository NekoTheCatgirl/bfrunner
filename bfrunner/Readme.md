# bfrunner

> A Brainfuck interpreter for shits and giggles. Seriously. Don’t use this in production.

`bfrunner` is a tiny, pure-Rust Brainfuck interpreter. It's fast enough to play with small programs, safe enough that it won't melt your computer (probably), and it comes with optional FFI support if you want to torture another language with this as well!

## Features

- ✅ Run Brainfuck code from strings or files
- ✅ Echo input/output with simple examples
- ✅ Detailed errors for unmatched brackets or pointer overflows
- ✅ Optional FFI interface for integration in other languages like C

## Installation

Add this to your `Cargo.toml`

```toml
[dependencies]
bfrunner = "0.1.0"
```

## Examples

### Hello world

```rust
use std::io::Cursor;
use bfrunner::{ BrainfuckError, run_to_string };

const SOURCE: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() -> Result<(), BrainfuckError> {
    let mut input = Cursor::new(Vec::<u8>::new());
    let output = run_to_string(SOURCE, &mut input)?;
    println!("{output}");
    Ok(())
}
```

### Echo Input

```rust
use std::io::Cursor;
use bfrunner::{ BrainfuckError, run_to_string };

const SOURCE: &str = ",[.,]";

fn main() -> Result<(), BrainfuckError> {
    let mut input = Cursor::new(b"Hello, world!".to_vec());
    let output = run_to_string(SOURCE, &mut input)?;
    println!("{output}");
    Ok(())
}
```

## Error Handling

`bfrunner` distinguishes between:

- **Parse errors** - unmatched brackets, invalid syntax
- **Runtime errors** - pointer out of bounds, I/O errors

## FFI Support (Optional)

If you build the library from source with the `ffi` feature enabled, you can run Brainfuck code from C using:

```c
int Brainfuck_run(const char*, const char*, char*, size_t);
```

Check the function in `ffi.rs` for error codes.

## Warning ⚠️

This crate exists purely for fun, learning, and mild frustration. Brainfuck is notoriously impractical, and so is this interpreter. Do not expect it to replace any serious programming language or runtime.

> Use at your own risk. May cause laughter, confusion, or existential dread.

## Contributing

PRs, bug reports, and memes are welcome. Keep it silly, keep it safe.
