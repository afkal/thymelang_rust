Thymelang
==========

Thymelang is an experimental language to evaluate Rust capabilities for building lexer, parser, interpreter and potentially also compiler in the longer run. Simultaneously Thymelang is a personal learning process to better understand computer languages and their capabilities and functionalities as well as better learn to utilize Rust in a professionally challenging environment.

Thymelang is in developmenet phase and consists of Rust binary application as well as library for lexer, parser and interpreter to demonstrate language functionalities. Application provides a command line interface for REPL interpreter and a library that can be used independently for creating a compiler with Rust.

Library contents
- lexer.rs : Lexer that tokenazes input stream to tokens
- parser.rs : Parser for creating Abstract Syntax Tree (AST) based on the parsed source
- type_checker: Type Checker that walks AST  and checks for the type declarations and creates symbol tree based on the types identified
- interpreter.rs : interpreter for AST (TBD)

Language structure
------------------

Thymelang operators and commands currently supported

- '+' operator (PLUS)
- '-' operator (MINUS)
- '*' operator (MUL)
- '/' operator (DIV)

Example thymelang contents (can be found from thymelang/examples/sample.thm):
```
/* Sample script file to demonstrate Thymelang feature */

// Comments may be either
/* blocks, or: */
// single line comments 

// Thyme is static strong typed language
// Variable definition: let VAR = VALUE SEMICOLON
// let is the keyword for defining variables
// These variables are GLOBAL since they are defined on the module level
// TBD: Shold Globals be actually supported at all???
// TBD with our without let keyword???
a_int = 5; // Integer
// or let a_int = 5;
a_float = 5.6; // Float
string = "test_string";
array = [1,2,3,4];

// Struct?
TestStruct = {
    a = 4;
    b = 3;
};

// function definition
let func = {
    a_int = a_int+1;
    b_float = a_float*2;
};

// function with arguments and return value
let sum_ints(a: int, b: int): int = {
    return(a+b);
};

// Program will start running from main()
let main() = {
    // print statement:
    print(sum_ints(2,3)); // 5
    // if statement
    if(a_int > 10) {
      print("a_int is bigger than 10");
    } else {
      print("a_int is smaller or equal to 10");
    }
};
```
