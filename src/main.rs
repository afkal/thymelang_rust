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
pub use thymelang::interpreter::{interpret};

/// Thymelang interpreter
#[derive(Clap)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in debug mode
    #[arg(short, action)]
    debug: bool,
    /// Source file path
    path: std::path::PathBuf,
}

fn main() {
    let _args = Args::parse();
    println!("Thymelang v0.10 (c) <proactor> 2022");
    println!("Type 'q' to Quit");
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

            // Testing tokenizer
            let mut tokenizer = Tokenizer::new(&input); // instantiate new lexer from source
            tokenizer.print_all_tokens();

            // Parsing 
            let mut parser = Parser::new(&input);
            let ast = parser.parse();
            println!("{:?}",ast);

            // Interpreter
            let result = interpret(ast);
            println!(": {}", result);

            /*
            Nim reference implementation:
            # Lexer
            echo "Lexer output:"
            var lexer = lexer.new(source) # instantiate new lexer from source
            #var token = lxr.getNextToken()
            lexer.getAllTokens()
            #echo token
      
            # Parser
            echo "Parser output:"
            var parser = parser.new(source)
            var ast = parser.parse()
            echo ast
      
            # Interpreter
            echo "Interpreter output:"
            var interpreter = interpreter.new(source)
            var result = interpreter.interpret()
            echo result
            */

        }
    }
}
