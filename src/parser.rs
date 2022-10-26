pub mod parser {

    use crate::lexer::lexer as lxr;

    pub struct Parser {
        /* Parse string into an AST */
        next_token: (String, String),
        tokenizer: lxr::Tokenizer
    }
    impl Parser {
    
        pub fn new(source: &str) -> Parser {
            Parser {
                next_token: ("".to_string(),"".to_string()), // empty token in the start
                tokenizer: lxr::Tokenizer::new(source)
            }
        }
    
        pub fn _get_next_token(&self) -> (String, String) {
            return self.next_token.clone();
        }
    
        pub fn parse(&mut self) -> (&str, String) {
    
            // Lookahead is a next token used for predictive parsing (LL1 parser)
            self.next_token = self.tokenizer.get_next_token();
            
            println!("Parsing token: ({}, {})", self.next_token.0, self.next_token.1);
    
            // Parse recursively
            return self.program(); // return the output of the runned program
        }

        /**
         * Consume current token and advance to next token
         */
        fn eat_token(&mut self, token_type : &str)-> (String, String) {

            let current_token = self.next_token.clone(); // Clone next token to current local token
            if self.next_token == ("".to_string(), "".to_string()) || self.next_token.0 != token_type {
                panic!("Expected token type: {}",token_type)
            }

            self.next_token = self.tokenizer.get_next_token(); // Advance lookahed to next token
            current_token // And return current token if there was a match
        }
    
        /**
         * Main entry point.
         * 
         * Program
         *   : Expression
         *   ;
         */
        fn program(&mut self) -> (&str, String) {
            return self.expression();
        }

        /**
         * Expression
         *   : Literal
         *   ;
         */
        fn expression(&mut self) -> (&str, String) {
            return self.literal();
            let left = self.literal();
            
        }

        /**
         * AdditiveExpression
         *   : Literal
         *   : AdditiveExpression ADDITIVE_OPERATOR Literal
         *   ;
         */
    
        /**
         * Literal
         *   : NumericLiteral
         *   | StringLiteral
         *   ;
         */
        fn literal(&mut self) -> (&str, String) {
            if self.next_token.0 == "NUMBER" {
                return self.numeric_literal();
            }
            if self.next_token.0 == "STRING" {
                return self.string_literal();
            }
            panic!("Unexpected literal!")
        }

        /**
         * NumericLiteral
         *   : NUMBER
         */
        fn numeric_literal(&mut self) -> (&str, String) {
            let token = self.eat_token("NUMBER");
            return ("NumericLiteral", token.1); // String to int cast
        }

        /**
         * StringLiteral
         *   : STRING
         */
        fn string_literal(&mut self) -> (&str, String) {
            let token = self.eat_token("STRING");
            return ("StringLiteral", token.1); // String to int cast
        }
    }
}