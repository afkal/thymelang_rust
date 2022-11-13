Thymelang
==========

Thymelang is an experimental language to evaluate Rust capabilities for building lexer, parser, interpreter and potentially also compiler in the longer run. Simultaneously Thymelang is a personal learning process to better understand computer languages and their capabilities and functionalities as well as better learn to utilize Rust in a professionally challenging environment.

Thymelang is in development phase and consists of Rust binary application as well as library for lexer, parser and interpreter to demonstrate language functionalities. Application provides a command line interface for REPL interpreter and a library that can be used independently for creating a compiler with Rust.

Library contents
- lexer.rs : Lexer that tokenazes input stream to tokens
- parser.rs : Parser for creating Abstract Syntax Tree (AST) based on the parsed source
- type_checker: Type Checker that walks AST  and checks for the type declarations and creates symbol tree based on the types identified
- interpreter.rs : Interpreter for AST (TBD)

Design principles
-----------------

Thyme lang is aimed to provide an easy syntax, compilable alternative for scripting languages such as Python. Thymelang is implemented by Rust and its syntax is also strongly inspired by Rust. One major driver is to provide easy tool for parallel processing, but parallellism is still not yet coverer by Thyme functionality.

Language structure
------------------

Operators and commands currently supported

Operators:
| Operator | Example | Description |
|  :----:  |  :----: |-------------|
| Binary operators |
| +        | 1+2     | Addition: term + term |
| -        | 1-2     | Substraction: term - term |
| *        | 1*2     | Multiplication:  factor * factor |
| /        | 1/2     | Integer division:  factor / factor |
| =        | a=1     | Assign operator: varible = expression |
| Unary Operators |
| +        | +2     | Positive: +factor |
| -        | -2     | Negatice: -factor |


Commands:
| Operator | Example | Description |
|  :----:  |  :----: |-------------|
| print()  | print(1);</br>print("something");</br>print(variableA);  | Echoes the contents of the value to the standard output |



Example thymelang contents (link to actual [file](/examples/sample.thm)):
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

Return values
-------------

Return value 0 for succesfull operation, and 1-255 represent faulty value.
