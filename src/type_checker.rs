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
/*
#[derive(Debug)]
pub struct Symbol {
    symbol_name: String,
    symbol_type: String
}
*/

pub struct TypeChecker {
    symbol_table: HashMap<String, String> // Global memory for variables and other structrures
}

impl TypeChecker {

    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new()
        }
    }

    pub fn echo_symbol_table(&self) {
        println!("{:?}", self.symbol_table);
    }

    /// Type check AST provided (by parser)
    pub fn evaluate(&mut self, ast: &Node) {
        self.visit_program(&ast);
    }

    // Get variable from symbol table and return its type
    fn visit_variable(&mut self, node: &Node) -> String {
        let result = self.symbol_table.get(&node.nvalue);
        match result {
            None => panic!("Type Error: Variable accessed before instatiotion: \"{}\".", node.nvalue),
            Some(var) => var.to_string(),
        }
        //return String::from("2");
    }

    // Binary op type is the type of its children (children must be of same type)
    fn visit_binaryop(&mut self, node: &Node) -> String {

        let left = self.visit(&node.children[0]);
        let right = self.visit(&node.children[1]);

        // If left type is same as right type, return that type
        if left == right {
            return left;
        }

        // else panic since both types are not equal
        panic!("Type Error: left and/or right of wrong type. Left {}, right {}", left, right);
    }

    // Unary op type is the type of its only child
    fn visit_unaryop(&mut self, node: &Node) -> String {

        // Type of unary op is the type of child node
        return self.visit(&node.children[0]);
    }

    /* Statements */

    fn visit_assignment_statement(&mut self, node: &Node) -> String {
        // Store value from the expression to variable
        let variable = node.children[0].nvalue.clone();
        let ntype = self.visit(&node.children[1]);

        println!("variable: {:?}, type: {:?}", variable, ntype);
        // Add variable to symbol table with value derived from the value
        self.symbol_table.insert(variable, ntype);
        return node.ntype.clone();
    }

    /// Print statement returns the value of expression on the child node
    fn visit_print_statement(&mut self, node: &Node) -> String {
        //self.visit(&node.children[0].clone());
        self.visit(&node.children[0]);
        return node.ntype.clone();
    }

    /* Program */
    /// Visit all children of Program Root node
    fn visit_program(&mut self, node: &Node) {

        for child in &node.children {
            self.visit(&child);
        }
    }

    /// Entry point for visiting nodes
    fn visit(&mut self, node: &Node) -> String {

        match node.ntype.as_str() {
            //"Program" => return self.visit_program(node),
            "PrintStatement" => return self.visit_print_statement(node),
            "AssignmentStatement" => return self.visit_assignment_statement(node),
            "Integer" | "Float" | "String" => return node.ntype.clone(), // Return literal type
            "UnaryOp" => return self.visit_unaryop(node),
            "AdditiveExpression" | "MultiplicationTerm" => return self.visit_binaryop(node),
            "Variable" => return self.visit_variable(node),
            ntype => return format!("Thyme Error: Could not run type check. Unknown type: \"{}\"", ntype),
        }
    }
}