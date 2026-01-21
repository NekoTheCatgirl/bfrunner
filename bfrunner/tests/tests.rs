use std::io::Cursor;
use bfrunner::*;

#[test]
fn hello_world_test() {
    let source =
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut input = Cursor::new(Vec::<u8>::new());
    let output = run_to_string(source, &mut input).expect("Execution failed");

    assert_eq!(output, "Hello World!\n")
}

#[test]
fn echo_test() {
    let source = ",[.,]";

    let mut input = Cursor::new(b"Hello!".to_vec());
    let output = run_to_string(source, &mut input).expect("Execution failed");

    assert_eq!(output, "Hello!")
}
