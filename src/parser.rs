use crate::lexer::Tokenizer;
use crate::lexer::Token;

#[derive(PartialEq, Debug, Clone)] // Enable formatted printing "{:?}"
pub struct Node {
    ntype: String,
    nvalue: String,
    children: Vec<Node>
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
        
        println!("Parsing token: ({}, {})", self.next_token.ttype, self.next_token.tvalue);

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

    /**
     * Main entry point.
     * 
     * Program
     *   : Expression
     *   ;
     */
    fn program(&mut self) -> Node {
        return self.expression();
    }

    ///
    /// Following example in: https://ruslanspivak.com/lsbasi-part7/
    /// Expression
    ///   : Term ((PLUS | MINUS) Term)*
    ///   ;
    fn expression(&mut self) -> Node {
        let mut left = self.term(); // Search for left term

        // TODO: Refactor to binary_operation
        //self.binary_operation()
        
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
        let mut left = self.literal(); // Initiate left literal

        while self.next_token.ttype == "MUL" || self.next_token.ttype == "DIV" {
            let current_operator = self.get_next_token();
            if current_operator.ttype == "MUL" { // Handle PLUS operator
                self.eat_token("MUL") // consume token
            } else if current_operator.ttype == "DIV" {
                self.eat_token("DIV") // consume token
            }
            let right = self.literal();
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
    //fn factor(mut &self) -> Node {

    /**
     * Literal
     *   : NumericLiteral
     *   | StringLiteral
     *   ;
     */
    fn literal(&mut self) -> Node {
        if self.next_token.ttype == "NUMBER" {
            return self.numeric_literal();
        }
        if self.next_token.ttype == "STRING" {
            return self.string_literal();
        }
        panic!("Unexpected literal!")
    }

    /**
     * NumericLiteral
     *   : NUMBER
     */
    fn numeric_literal(&mut self) -> Node {
        let token = self.get_next_token();
        self.eat_token("NUMBER");
        /*
        return Node {
            ntype: String::from("NumericLiteral"),
            nvalue: token.tvalue
        }
        */
        Node::new_without_children("NumericLiteral", &token.tvalue)
    }

    /**
     * StringLiteral
     *   : STRING
     */
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
        let expected = Node::new_without_children("NumericLiteral", "153");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_single_string() {
        let mut parser = Parser::new("\"Testing\"");
        let result = parser.parse();
        let expected = Node::new_without_children("StringLiteral", "\"Testing\"");
        assert_eq!(expected, result);
    }
}