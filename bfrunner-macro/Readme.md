# bfrunner-macro

> Now you are off the deep end

This is no longer funny now you want to run Brainfuck at compile time???

Well sure whatever, heres how:

```rust
use bfrunner_macro::bf;

fn main() {
    let output = bf!(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
    );
    println!("{output}");
}
```
