use std::io::Cursor;

use bfrunner::{ BrainfuckError, run_to_string };

const SOURCE: &str = ",[.,]";

fn main() -> Result<(), BrainfuckError> {
    let mut input = Cursor::new(b"Hello!".to_vec());
    let output = run_to_string(SOURCE, &mut input)?;
    println!("{output}");
    Ok(())
}
