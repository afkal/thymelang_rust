/*
 * Compiler PoC based on LLVM
 * Reference: https://github.com/jauhien/iron-kaleidoscope#basic-variant-of-the-kaleidoscope-language
 * Book with Rust: https://createlang.rs/
 */

//use std::env;
use std::fs; // File system
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


fn execute(source: String, mut interpreter: Interpreter, mut type_checker: TypeChecker, debug_mode: bool ) {
    // Tokenize source code
    if debug_mode {
        println!("Tokenizer output:");
        let mut tokenizer = Tokenizer::new(&source); // instantiate new lexer from source
        tokenizer.print_all_tokens(); // Tokenizer debug output
    }

    // Create Asymmetric Symbol Tree from input source
    let mut parser = Parser::new(&source);
    let ast = parser.parse();

    if debug_mode {
        println!("AST generated:");
        println!("{:?}",ast); // Parser debug output
    }

    // Run type checker for static type checking
    type_checker.evaluate(&ast);
    let stable = type_checker.get_symbol_table(); // Debug output
    if debug_mode {
        println!("Symbol table:");
        println!("{:?}",stable); // Type Chekcer debug output
    }

    // Interpret AST provided by parser
    let result = interpreter.interpret(ast);
    if debug_mode {
        interpreter.echo_global_memory(); // Debug output
    }

    // Print result
    println!(": {}", result);

}

fn main() {

    let args = Args::parse();

    // Instantiate interpreter with global memory.
    // For REPL Interpreter shall remain same during whole REPL session
    // Thus also symbol table remembers results between multiple inputs
    let mut interpreter = Interpreter::new();
    // Same applies for type checker and symbol table generated
    let mut type_checker = TypeChecker::new();

    println!("Thymelang v0.10 (c) <proactor> 2022");
    println!("Type 'quit()' to Quit");

    // Handle case when file path provided
    match args.path {
        Some(file_path) => {
            println!("Executing file: \"{}\"", file_path.display());
            let source = fs::read_to_string(file_path)
                .expect("Thyme Error: Input file not found!");

            //println!("File contents:\n{contents}");
            execute(source, interpreter, type_checker, args.debug);
            return;
        },
        None => {},
    };
 
    // If not file provided start REPL loop
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

            // Tokenize source code
            if args.debug {
                println!("Tokenizer output:");
                let mut tokenizer = Tokenizer::new(&input); // instantiate new lexer from source
                tokenizer.print_all_tokens(); // Tokenizer debug output
            }

            // Create Asymmetric Symbol Tree from input source
            let mut parser = Parser::new(&input);
            let ast = parser.parse();

            if args.debug {
                println!("AST generated:");
                println!("{:?}",ast); // Parser debug output
            }

            // Run type checker for static type checking
            type_checker.evaluate(&ast);
            println!("Symbol table:");
            let stable = type_checker.get_symbol_table(); // Debug output
            println!("{:?}",stable); // Type Chekcer debug output

            // Interpret AST provided by parser
            let result = interpreter.interpret(ast);
            interpreter.echo_global_memory(); // Debug output
            // Print result
            println!(": {}", result);
        }
    }
}
