use std::{ io, isize, usize };

use thiserror::Error;

#[cfg(feature = "ffi")]
mod ffi;

#[cfg(feature = "compile_time_macro")]
pub use bfrunner_macro::bf;

const TAPE_SIZE: usize = 30000;

#[derive(Debug, Clone)]
pub struct Tape {
    tape: [u8; TAPE_SIZE],
    dp: usize,
}

impl Default for Tape {
    fn default() -> Self {
        Self { tape: [0; TAPE_SIZE], dp: 0 }
    }
}

#[derive(Debug, Clone)]
pub enum AST {
    Move(isize),
    Add(i8),
    Output,
    Input,
    Loop(Vec<AST>),
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Data pointer went out of bounds")]
    PointerOutOfBounds,
    #[error("I/O Error")] Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unmatched closing bracket ']' at position {pos}")] UnmatchedClosingBracket {
        pos: usize,
    },

    #[error("Unmatched opening bracket '[' (opened at position {pos})")] UnmatchedOpeningBracket {
        pos: usize,
    },
}

#[derive(Debug, Error)]
pub enum BrainfuckError {
    #[error(transparent)] Parse(#[from] ParseError),
    #[error(transparent)] Runtime(#[from] RuntimeError),
    #[error(transparent)] String(#[from] std::string::FromUtf8Error),
}

/// Runs a Brainfuck program and returns its output.
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use bfrunner::run_to_string;
///
/// let source = "+++.";
/// let mut input = Cursor::new(Vec::new());
/// let output = run_to_string(source, &mut input).unwrap();
///
/// assert_eq!(output.as_bytes(), &[3]);
/// ```
pub fn run_to_string<R: io::Read>(source: &str, input: &mut R) -> Result<String, BrainfuckError> {
    let ast = parse(source)?;
    let mut state = Tape::default();

    let mut output = Vec::new();

    exec(&ast, &mut state, input, &mut output)?;

    let result = String::from_utf8(output)?;

    Ok(result)
}

pub fn exec<R: io::Read, W: io::Write>(
    instrs: &[AST],
    state: &mut Tape,
    input: &mut R,
    output: &mut W
) -> Result<(), RuntimeError> {
    for instruction in instrs {
        match instruction {
            AST::Move(n) => {
                let new_dp = (state.dp as isize) + n;
                if new_dp < 0 || new_dp >= (TAPE_SIZE as isize) {
                    return Err(RuntimeError::PointerOutOfBounds);
                }
                state.dp = new_dp as usize;
            }
            AST::Add(n) => {
                state.tape[state.dp] = state.tape[state.dp].wrapping_add(*n as u8);
            }
            AST::Output => {
                output.write_all(&[state.tape[state.dp]])?;
            }
            AST::Input => {
                let mut buf = [0u8];
                let read = input.read(&mut buf)?;
                state.tape[state.dp] = if read == 0 { 0 } else { buf[0] };
            }
            AST::Loop(body) => {
                while state.tape[state.dp] != 0 {
                    exec(&body, state, input, output)?;
                }
            }
        }
    }
    Ok(())
}

pub fn parse(source: &str) -> Result<Vec<AST>, ParseError> {
    let mut stack: Vec<(Vec<AST>, usize)> = vec![(vec![], 0)];
    let mut chars = source.char_indices().peekable();

    while let Some((pos, c)) = chars.next() {
        match c {
            '>' | '<' => {
                let mut n = if c == '>' { 1 } else { -1 };
                while matches!(chars.peek(), Some((_, '>')) | Some((_, '<'))) {
                    n += if chars.next().unwrap().1 == '>' { 1 } else { -1 };
                }
                stack.last_mut().unwrap().0.push(AST::Move(n));
            }
            '+' | '-' => {
                let mut n = if c == '+' { 1 } else { -1 };
                while matches!(chars.peek(), Some((_, '+')) | Some((_, '-'))) {
                    n += if chars.next().unwrap().1 == '+' { 1 } else { -1 };
                }
                stack.last_mut().unwrap().0.push(AST::Add(n));
            }
            '.' => stack.last_mut().unwrap().0.push(AST::Output),
            ',' => stack.last_mut().unwrap().0.push(AST::Input),
            '[' => stack.push((Vec::new(), pos)),
            ']' => {
                let (body, _) = stack.pop().ok_or(ParseError::UnmatchedClosingBracket { pos })?;

                stack.last_mut().unwrap().0.push(AST::Loop(body));
            }
            _ => {} // Comment
        }
    }

    if stack.len() > 1 {
        let (_, pos) = stack.pop().unwrap();
        return Err(ParseError::UnmatchedOpeningBracket { pos });
    }

    Ok(stack.pop().unwrap().0)
}
