/*
 * Compiler PoC based on LLVM
 * Reference: https://github.com/jauhien/iron-kaleidoscope#basic-variant-of-the-kaleidoscope-language
 * Book with Rust: https://createlang.rs/
 */
//use compiler_poc::lexer; /* start poc with lexer functionality */
use std::env;
//mod lexer;
//mod parser;
pub use thymelang_rust::lexer::Tokenizer;
pub use thymelang_rust::parser::Parser;

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

            // Testing tokenizer
            /* Old implementation works against "lexer_old.rs"
            let mut tokenizer = lxr::Tokenizer::new(&input); // instantiate new lexer from source
            tokenizer.print_all_tokens();
            */

            let mut tokenizer = Tokenizer::new(&input); // instantiate new lexer from source
            tokenizer.print_all_tokens();

            // Parsing 
            let mut parser = Parser::new(&input);
            let result = parser.parse();
            println!("{:?}",result);


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
