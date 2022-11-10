use crate::parser::Node;
use std::collections::HashMap;

/// Hashmap implementation still to be designed how
pub struct Interpreter {
    global_memory: HashMap<String, String> // Global memory for variables and other structrures
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            global_memory: HashMap::new()
        }
    }

    /// Interpret AST provided (by parser)
    pub fn interpret(&mut self, ast: Node) -> String {
        return self.visit(ast);
    }

    fn visit_variable(&mut self, node: Node) -> String {
        let result = self.global_memory.get(&node.nvalue);
        match result {
            None => panic!("Parser Error: Variable \"{}\" not instantiated.", node.nvalue),
            Some(var) => return var.to_string(),
        }
        //return String::from("2");
    }

    fn visit_assignment_statement(&mut self, node: Node) -> String {
        // Store value from the expression to variable
        let variable = node.children[0].nvalue.clone();
        let value = self.visit(node.children[1].clone());
        self.global_memory.insert(variable, value.clone());
        return value;
        //return format!("AssingmentStatement Not yet implemented for \"{} = {}\"", variable, value);
    }

    /// Print statement returns the value of expression on the child node
    fn visit_print_statement(&mut self, node: Node) -> String {
        let value = self.visit(node.children[0].clone());
        return value;
    }

    // TODO: Only integer supported for now...
    fn visit_binaryop(&mut self, node: Node) -> String {

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

    fn visit_unaryop(&mut self, node: Node) -> String {

        /* In case unaryop is "+" return visited child node */
        if node.nvalue == "+" {
            return self.visit(node.children[0].clone());
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

    /// Visit all children of Program Root node
    fn visit_program(&mut self, node: Node) -> String {

        let mut result = String::from("");
        for child in node.children {
            result = self.visit(child);
        }
        /* Currently only last result (=source code line) is returned */
        return result;
    }

    /// Entry point for interpreting program
    fn visit(&mut self, node: Node) -> String {

        match node.ntype.as_str() {
            "Program" => return self.visit_program(node),
            "PrintStatement" => return self.visit_print_statement(node),
            "IntegerLiteral" | "FloatLiteral" | "StringLiteral" => return node.nvalue,
            "UnaryOp" => return self.visit_unaryop(node),
            "AdditiveExpression" | "MultiplicationTerm" => return self.visit_binaryop(node),
            "AssignmentStatement" => return self.visit_assignment_statement(node),
            "Variable" => return self.visit_variable(node),
            ntype => return format!("Thyme Error: Could not interpret. Unknown type: \"{}\"", ntype),
        }
    }
}