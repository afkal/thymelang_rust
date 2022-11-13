/*
 * Compiler PoC based on LLVM
 * Reference: https://github.com/jauhien/iron-kaleidoscope#basic-variant-of-the-kaleidoscope-language
 * Book with Rust: https://createlang.rs/
 */

//use std::env;
use clap::Parser as Clap;
use std::io::{self, Write};
// Thymelang modules
pub use thymelang::lexer::Tokenizer;
pub use thymelang::parser::Parser;
pub use thymelang::type_checker::TypeChecker;
pub use thymelang::interpreter::Interpreter;

/// Thymelang interpreter
#[derive(Clap)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in debug mode
    #[arg(short, action)]
    debug: bool, // optional argument (boolean)
    /// Source file path
    path: Option<std::path::PathBuf>, // Made optional so that program can be run without filepath
}

fn main() {

    let args = Args::parse();
    match args.path {
        Some(path) => {
            println!("Path \"{}\" provided. Running script from file not yet supported.", path.display());
            return;
        },
        None => {},
    };
    println!("Thymelang v0.10 (c) <proactor> 2022");
    println!("Type 'quit()' to Quit");

    // Instantiate interpreter with global memory.
    // Interpreter shall remain same during whole REPL session
    // Thus also symbol table remembers results between multiple inputs
    let mut interpreter = Interpreter::new();
    // Same applies for type checker and symbol table generated
    let mut type_checker = TypeChecker::new();


    // Start REPL loop
    loop {
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

            // Run type checker for static type checking
            type_checker.evaluate(&ast);
            println!("Symbol table:");
            type_checker.echo_symbol_table(); // Debug output

            // Interpret AST provided by parser
            let result = interpreter.interpret(ast);
            interpreter.echo_global_memory(); // Debug output
            // Print result
            println!(": {}", result);
        }
    }
}
