//! Type Checker checks for the declarations and types for the program items
//! type checking is performed against AST provided by program parser
//! 
//! Type Checker supports checking following items:
//! * ...
//! * ...
//!
//! Type checker also builds the symbol tree for possible further use

use crate::parser::Node;
use std::collections::HashMap;


pub struct Symbol {
    symbol_name: String,
    symbol_type: String
}


pub struct TypeChecker {
    symbol_table: HashMap<String, Symbol> // Global memory for variables and other structrures
}

impl TypeChecker {

    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new()
        }
    }

    /// Interpret AST provided (by parser)
    pub fn evaluate(&mut self, ast: &Node) {
        self.visit(&ast);
    }
    /*
    fn visit_variable(&mut self, node: Node) {
        let result = self.global_memory.get(&node.nvalue);
        match result {
            None => panic!("Parser Error: Variable \"{}\" not instantiated.", node.nvalue),
            Some(var) => var.to_string(),
        }
        //return String::from("2");
    }

    // TODO: Only integer supported for now...
    fn visit_binaryop(&mut self, node: Node) {

        let left_as_string = self.visit(node.children[0].clone());
        let right_as_string = self.visit(node.children[1].clone());

        let left: i32;
        let right: i32;

        match left_as_string.parse::<i32>() {
            Ok(n) => left = n,
            Err(e) => panic!("Parse error: Could not convert {} to int. Error received while parsing: {}", left_as_string, e),
        }
        match right_as_string.parse::<i32>() {
            Ok(n) => right = n,
            Err(e) => panic!("Parse error: Could not convert {} to int. Error received while parsing: {}", right_as_string, e),
          }

        match node.nvalue.as_str() {
            "+" => return (left+right).to_string(),
            "-" => return (left-right).to_string(),
            "*" => return (left*right).to_string(),
            "/" => return (left/right).to_string(),
            val => panic!("Invalid binary operator: \"{}\"", val)
        }
    }

    fn visit_unaryop(&mut self, node: Node) {

        /* In case unaryop is "+" return visited child node */
        if node.nvalue == "+" {
            self.visit(node.children[0].clone());
        }

        match node.children[0].ntype.as_str() {
            "IntegerLiteral" => {
                let val = node.children[0].nvalue.clone();
                let val_int = val.parse::<i32>().unwrap();
                return (-1 * val_int).to_string();
            },
            "FloatLiteral" => {
                let val = node.children[0].nvalue.clone();
                let val_float = val.parse::<f32>().unwrap();
                return (-1.0 * val_float).to_string();
            },
            "Variable" | "UnaryOp" | "MultiplicationTerm" | "AdditiveExpression" => {
                let val = self.visit(node.children[0].clone());
                let val_int = val.parse::<i32>().unwrap();
                return (-1 * val_int).to_string();
            },
            _ => panic!("UnaryOperation not supported for type: \"{}\". Error occurred with item: {}", node.children[0].ntype, node.children[0].nvalue),
        }
    }

    /* Statements */

    fn visit_assignment_statement(&mut self, node: Node) {
        // Store value from the expression to variable
        let variable = node.children[0].nvalue.clone();
        let value = self.visit(node.children[1].clone());
        self.global_memory.insert(variable, value.clone());
        value;
        //return format!("AssingmentStatement Not yet implemented for \"{} = {}\"", variable, value);
    }
    */

    /// Print statement returns the value of expression on the child node
    fn visit_print_statement(&mut self, node: &Node) {
        self.visit(&node.children[0].clone());
    }

    /* Program */
    /// Visit all children of Program Root node
    fn visit_program(&mut self, node: &Node) {

        let mut result = String::from("");
        for child in &node.children {
            self.visit(&child);
        }
    }

    /// Entry point for interpreting program
    fn visit(&mut self, node: &Node) {

        match node.ntype.as_str() {
            "Program" => self.visit_program(node),
            "PrintStatement" => self.visit_print_statement(node),
            //"IntegerLiteral" | "FloatLiteral" | "StringLiteral" => node.nvalue,
            //"UnaryOp" => self.visit_unaryop(node),
            //"AdditiveExpression" | "MultiplicationTerm" => self.visit_binaryop(node),
            //"AssignmentStatement" => self.visit_assignment_statement(node),
            //"Variable" => self.visit_variable(node),
            ntype => panic!("Thyme Error: Could not run type check. Unknown type: \"{}\"", ntype),
        }
    }
}