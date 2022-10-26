/*
 * Compiler PoC based on LLVM
 * Reference: https://github.com/jauhien/iron-kaleidoscope#basic-variant-of-the-kaleidoscope-language
 * Book with Rust: https://createlang.rs/
 */
mod lexer; /* start poc with lexer functionality */
use std::io::{self, Write};

fn main() {
    println!("Rust compiler/interpreter PoC. Type 'q' to Quit");
    //let mut input;
    loop {
        //input = "".to_string();
        let mut input = String::new();
        print!("{}","thyme> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input == "q\n" {
            break; // Press 'q' to quit'
        } else {
            let result = lexer::tokenize(&mut input);
            for (name, value) in result {
                print!("({}:{})", name, value);
            }
            println!("");
        }
    }
}
