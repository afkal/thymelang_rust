Thymelang
==========

Thymelang is an experimental language to evaluation Rust capabilities for building lexer, parser, interpreter and potentially also compiler in the longer run. Simultaneously Thymelang is a personal learning process to better understand computer languages and their capabilities and functionalities as well as better learn to utilize Rust in a professionally challenging environment.

Thymelang is in developmenet phase and consists of Rust binary application as well as library for lexer and parser to demonstrate compiler / interpreter functionalities. Application provides a command line interface for JIT interpreter and a library that can be used independently for creating a compiler with Rust.

Library contents
- lexer.rs : lexer for tokenazing input stream
- parser.rs : parser for creating Abstract Syntax Tree (AST) based on the parsed source
- interpreter.rs : interpreter for AST (TBD)

Thymelang operators and commands currently supported

- '+' operator (PLUS)
- '-' operator (MINUS)
- '*' operator (MUL)
- '/' operator (DIV)
