/*
 * Compiler PoC based on LLVM
 * Reference: https://github.com/jauhien/iron-kaleidoscope#basic-variant-of-the-kaleidoscope-language
 * Book with Rust: https://createlang.rs/
 */
//use compiler_poc::lexer; /* start poc with lexer functionality */
use std::env;
mod lexer;
mod parser;
pub use crate::lexer::lexer as lxr;
pub use crate::parser::parser as psr;

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
            let mut tokenizer = lxr::Tokenizer::new(&input); // instantiate new lexer from source
            tokenizer.print_all_tokens();

            // Parsing 
            let mut parser = psr::Parser::new(&input);
            let result = parser.parse();
            println!("{:?}",result);
            /*
            let result = lexer::tokenize(&mut input);
            for (name, value) in result {
                print!("({}:{})", name, value);
            }
            println!("");
            */
            // To be refaktored to object oriented step by step mode:
            /*
            let lexer = Lexer::new(input); // returns new lexer object
            //tokens = lexer::make_tokens() // optionally create a list of tokens (not needed in step-by-step mode)
            let parser = Parser::new(lexer); // returns new parser object
    
            let mut interpreter = Interpreter::new(parser); // returns new interpreter object
            let result = interpreter.interpret(); // interpreter output
            println!("{}", result);
            */


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
