/*
 * Compiler PoC based on LLVM
 * Reference: https://github.com/jauhien/iron-kaleidoscope#basic-variant-of-the-kaleidoscope-language
 * Book with Rust: https://createlang.rs/
 */

//use std::env;

pub use thymelang_rust::lexer::Tokenizer;
pub use thymelang_rust::parser::Parser;
pub use thymelang_rust::interpreter::{interpret};

use std::io::{self, Write};

fn main() {
    println!("Thymelang interpreter v0.1. Type 'q' to Quit");
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
            interpret(ast);

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
