/*
 * Compiler PoC based on LLVM
 * Reference: https://github.com/jauhien/iron-kaleidoscope#basic-variant-of-the-kaleidoscope-language
 * Book with Rust: https://createlang.rs/
 */

//use std::env;
use clap::Parser as Clap;
use std::io::{self, Write};
pub use thymelang::lexer::Tokenizer;
pub use thymelang::parser::Parser;
pub use thymelang::interpreter::Interpreter;

/// Thymelang interpreter
#[derive(Clap)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in debug mode, optional argument (boolean)
    #[arg(short, action)]
    debug: bool,
    /// Source file path
    path: Option<std::path::PathBuf>, // Made optional so that program can be run without filepath
}

fn main() {
    let args = Args::parse();
    println!("Thymelang v0.10 (c) <proactor> 2022");
    println!("Type 'q' to Quit");
    //let mut input;

    // Instantiate interpreter with symbol table.
    // Interpreter shall remain same during whole REPL session
    // Thus also symbol table remembers results between multiple inputs
    let mut interpreter = Interpreter::new();

    // Start REPL loop
    loop {
        //input = "".to_string();
        let mut input = String::new();
        print!("{}","thyme> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input == "quit()\n" {
            break; // Press 'q' to quit'
        } else {

            // Create Asymmetric Symbol Tree from input source
            let mut parser = Parser::new(&input);
            let ast = parser.parse();

            if args.debug {
                println!("Tokenizer output:");
                let mut tokenizer = Tokenizer::new(&input); // instantiate new lexer from source
                tokenizer.print_all_tokens(); // Tokenizer debug output
                println!("AST generated:");
                println!("{:?}",ast); // Parser debug output
            }
            
            // Interpret AST provided by parser
            let result = interpreter.interpret(ast);
            // Print result
            println!(": {}", result);
        }
    }
}
