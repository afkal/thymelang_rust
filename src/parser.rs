use std::fmt;
use serde::Serialize;
use crate::lexer::Tokenizer;
use crate::lexer::Token;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Node {
    pub ntype: String,
    pub nvalue: String,
    pub children: Vec<Node>
}

// Implement display formatter for pretty printing Nodes
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ntype: {}", self.ntype)
    }
}

impl Node {
    pub fn new(ntype: &str, nvalue: &str, children: Vec<Node>) -> Self {
        Self {
            ntype: String::from(ntype),
            nvalue: String::from(nvalue),
            children
        }
    }
                
    pub fn new_without_children(ntype: &str, nvalue: &str) -> Self {
        Self {
            ntype: String::from(ntype),
            nvalue: String::from(nvalue),
            children: Vec::new()
        }
    }
}

pub struct Parser {
    /* Parse string into an AST */
    next_token: Token,
    tokenizer: Tokenizer
}
impl Parser {

    pub fn new(source: &str) -> Parser {
        Parser {
            // Initialize with empty token in the start
            next_token: Token {
                ttype: String::from(""),
                tvalue: String::from("")
            },
            tokenizer: Tokenizer::new(source)
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        self.next_token.clone()
    }
    
    pub fn parse(&mut self) -> Node {

        // Lookahead is a next token used for predictive parsing (LL1 parser)
        self.next_token = self.tokenizer.get_next_token();
        
        //println!("Parsing token: ({}, {})", self.next_token.ttype, self.next_token.tvalue);

        // Parse recursively
        return self.program(); // return the output of the runned program
    }

    ///
    /// Consume current token and get next token from input stream
    ///
    fn eat_token(&mut self, token_type : &str) {

        //let current_token = self.next_token.clone(); // Clone next token to current local token
        if self.next_token.ttype == "".to_string() || self.next_token.ttype != token_type {
            panic!("Expected token type: {}",token_type)
        }

        self.next_token = self.tokenizer.get_next_token(); // Advance lookahed to next token
        //current_token // And return current token if there was a match
    }

    ///
    /// Main entry point.
    /// 
    /// Program
    ///   : Statementlist
    ///   ;
    ///
    fn program(&mut self) -> Node {
        let children = self.statementlist();
        return Node::new("Program", "Root", children);
    }

    /// Statementlist, list of statements divided by semicolons
    /// Statementlist
    ///   : Statement SEMICOLON Statementlist
    ///   ;
    fn statementlist(&mut self) -> Vec<Node> {
        let mut statements : Vec<Node> = Vec::new();
        let node = self.statement(); // Get result from first statement
        statements.push(node);
        // Loop while SEMICOLON(s) found at the end of statement
        while self.get_next_token().ttype == "SEMICOLON" {
            self.eat_token("SEMICOLON");
            // Add new statement to the list in case no end-of-file was reached
            if self.get_next_token().ttype != "EOF" {
                statements.push(self.statement());
            }
        }
        return statements;
    }

    ///  Statement
    ///   : Assignment statement
    ///   | Print statement
    ///   | Expression
    ///   ;
    fn statement(&mut self) -> Node {
        // Assignment statement starts with Identifier (Variable name)
        if self.get_next_token().ttype == "IDENTIFIER" {
            return self.assignment_statement();
        } else if self.get_next_token().ttype == "PRINT" {
            return self.print_statement();
        }
        // If nothing above - expect pure expressions (temporarely supported)
        return self.expression();
    }

    /// PrintStatement
    ///   : PRINT LPAREN expression RPAREN
    ///   ;
    fn print_statement(&mut self) -> Node {
        let command = self.get_next_token();
        self.eat_token("PRINT"); // Expect PRINT command
        let _node = self.get_next_token();
        self.eat_token("LPAREN"); // Expect LPAREN
        let child = self.expression(); // Get expression from right
        let _node = self.get_next_token();
        self.eat_token("RPAREN"); // Expect RPAREN
        return Node::new("PrintStatement", &command.tvalue, Vec::from([child]));
    }

    /// Assignment statement
    ///   : Variable ASSIGN Expression
    ///   ;
    fn assignment_statement(&mut self) -> Node { 
        // Find variable as left token
        let left = self.variable();
        let oper = self.get_next_token();
        self.eat_token("ASSIGN"); // Expect ASSIGN operator
        let right = self.expression();
        let result = Node::new("AssignmentStatement", &oper.tvalue, Vec::from([left, right]));
        return result;
    }

    /// Expression
    ///   : Term ((PLUS | MINUS) Term)*
    ///   ;
    fn expression(&mut self) -> Node {
        let mut left = self.term(); // Search for left term

        // TODO: Refactor to binary_operation
        //left = self.binary_operation(&left, "AdditiveExpression");
        
        while self.next_token.ttype == "PLUS" || self.next_token.ttype == "MINUS" {
            let current_operator = self.get_next_token();
            if current_operator.ttype == "PLUS" { // Handle PLUS operator
                self.eat_token("PLUS") // consume token
            } else if current_operator.ttype == "MINUS" {
                self.eat_token("MINUS") // consume token
            }
            let right = self.term();
            left = Node {
                ntype: String::from("AdditiveExpression"),
                nvalue: current_operator.tvalue,
                children: Vec::from([left, right])
            }
        }
        return left; // return either single term or result of chain of additive expressions if present
        
    }

    /// Term
    ///   : Factor ((MUL | DIV) Factor)*
    ///   ;
    fn term(&mut self) -> Node {
        // TODO
        let mut left = self.factor(); // Initiate left literal

        if self.next_token.ttype == "ERROR" {
            panic!("Parser error: Invalid operator!");
        }

        while self.next_token.ttype == "MUL" || self.next_token.ttype == "DIV" {
            let current_operator = self.get_next_token();
            if current_operator.ttype == "MUL" { // Handle PLUS operator
                self.eat_token("MUL") // consume token
            } else if current_operator.ttype == "DIV" {
                self.eat_token("DIV") // consume token
            }
            let right = self.factor();
            left = Node {
                ntype: String::from("MultiplicationTerm"),
                nvalue: current_operator.tvalue,
                children: Vec::from([left, right])
            }
        }
        return left; // return either single literal or result of chain of multiplication expressions if present

    }

    /// Factor
    ///  : PLUS Factor
    ///  | MINUS Factor
    ///  | Literal
    ///  | LPAREN expr RPAREN
    ///  | Variable
    ///  ;
    fn factor(&mut self) -> Node {
        let token = self.get_next_token();

        // Handle parenthesis "( __ )"
        if token.ttype == "LPAREN" {
            self.eat_token("LPAREN"); // Consume token left parenthesis "("
            let node = self.expression(); // Handle expression between parenthesis
            self.eat_token("RPAREN"); // Consume token right parenthesis ")"
            return node;
        }
        // Handle unary operator PLUS
        if token.ttype == "PLUS" {
            self.eat_token("PLUS"); // Consume token
            let mut children : Vec<Node> = Vec::new();
            children.push(self.factor()); // Get factor from the right hand side
            return Node::new("UnaryOp", &token.tvalue, children);
        }
        // Handle unary operator MINUS
        if token.ttype == "MINUS" {
            self.eat_token("MINUS"); // Consume token
            let mut children : Vec<Node> = Vec::new();
            children.push(self.factor()); // Get factor from the right hand side
            return Node::new("UnaryOp", &token.tvalue, children);
        }
        // Handle IDENTIFIER
        if token.ttype == "IDENTIFIER" {
            return self.variable();
        }
        // Handle LITERAL
        return self.literal();
    }

    /// Variables are mutable, i.e., their values can be changed and updated.
    fn variable(&mut self) -> Node {
        let token = self.get_next_token();
        self.eat_token("IDENTIFIER"); // Expect identifier token
        return Node::new_without_children("Variable", &token.tvalue)
    }

    ///
    /// Literal
    ///   : NumericLiteral
    ///   | StringLiteral
    ///   ;
    ///
    fn literal(&mut self) -> Node {
        match self.next_token.ttype.as_str() {
            "INT_NUMBER" | "FLOAT_NUMBER" => return self.numeric_literal(),
            "STRING" => return self.string_literal(),
            value => panic!("Error: unexpected literal: '{}', expected String or Number.", value),
        }
    }

    /// NumericLiteral
    ///   : IntegerLiteral
    ///   | FloatLiteral
    ///   ;
    fn numeric_literal(&mut self) -> Node {
        match self.next_token.ttype.as_str() {
            "INT_NUMBER" => return self.integer_literal(),
            "FLOAT_NUMBER" => return self.float_literal(),
            value => panic!("Error: unexpected number: '{}', expected int or float.", value),
        }
    }

    /// IntegerLiteral
    ///   : INT_NUMBER
    ///   ;
    fn integer_literal(&mut self) -> Node {
        let token = self.get_next_token();
        self.eat_token("INT_NUMBER");
        Node::new_without_children("IntegerLiteral", &token.tvalue)
    }

    /// FloatLiteral
    ///   : FLOAT_NUMBER
    ///   ;
    fn float_literal(&mut self) -> Node {
        let token = self.get_next_token();
        self.eat_token("FLOAT_NUMBER");
        Node::new_without_children("FloatLiteral", &token.tvalue)
    }

    ///
    /// StringLiteral
    ///   : STRING
    ///
    fn string_literal(&mut self) -> Node {
        let token = self.get_next_token();
        self.eat_token("STRING");
        Node::new_without_children("StringLiteral", &token.tvalue)
    }
}

#[cfg(test)]
mod tests {

    use crate::parser::Parser;
    use crate::parser::Node;

    #[test]
    fn test_parse_single_integer() {
        let mut parser = Parser::new("153");
        let result = parser.parse();
        let expected = Node::new_without_children("IntegerLiteral", "153");
        assert_eq!(expected, result.children[0]);
    }

    #[test]
    fn test_parse_single_float() {
        let mut parser = Parser::new("153.234");
        let result = parser.parse();
        let expected = Node::new_without_children("FloatLiteral", "153.234");
        assert_eq!(expected, result.children[0]);
    }

    #[test]
    fn test_parse_single_string() {
        let mut parser = Parser::new("\"Testing\"");
        let result = parser.parse();
        let expected = Node::new_without_children("StringLiteral", "\"Testing\"");
        assert_eq!(expected, result.children[0]);
    }

    /* Single identifier not currently supported by parser */
    /*
    #[test]
    fn test_parse_identifier() {
        let mut parser = Parser::new("aaVa_12");
        let result = parser.parse();
        let expected = Node::new_without_children("Variable", "aaVa_12");
        assert_eq!(expected, result.children[0]);
    }
    */
}