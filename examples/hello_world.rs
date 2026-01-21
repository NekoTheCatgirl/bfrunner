use std::io::Cursor;

use bfrunner::{ BrainfuckError, run_to_string };

const SOURCE: &str = include_str!("./hello_world.bf");

fn main() -> Result<(), BrainfuckError> {
    let mut input = Cursor::new(Vec::<u8>::new());
    let output = run_to_string(SOURCE, &mut input)?;
    println!("{output}");
    Ok(())
}
