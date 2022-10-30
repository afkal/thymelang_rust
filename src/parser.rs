pub mod parser {

    use crate::lexer::lexer as lxr;
    use crate::lxr::Token;

    #[derive(Debug)] // Enable formatted printing "{:?}"
    pub struct Node {
        ntype: String,
        nvalue: String,
        children: Vec<Node>
    }

    impl Node {
        pub fn new_without_children(ntype: &str, nvalue: String) -> Self {
            Self {
                ntype: String::from(ntype),
                //nvalue: String::from(nvalue),
                nvalue: nvalue,
                children: Vec::new()
            }
        }
    }

    pub struct Parser {
        /* Parse string into an AST */
        next_token: Token,
        tokenizer: lxr::Tokenizer
    }
    impl Parser {
    
        pub fn new(source: &str) -> Parser {
            Parser {
                // Initialize with empty token in the start
                next_token: Token {
                    ttype: String::from(""),
                    tvalue: String::from("")
                },
                tokenizer: lxr::Tokenizer::new(source)
            }
        }
        
        pub fn parse(&mut self) -> Node {
    
            // Lookahead is a next token used for predictive parsing (LL1 parser)
            self.next_token = self.tokenizer.get_next_token();
            
            println!("Parsing token: ({}, {})", self.next_token.ttype, self.next_token.tvalue);
    
            // Parse recursively
            return self.program(); // return the output of the runned program
        }

        /**
         * Consume current token and advance to next token
         */
        fn eat_token(&mut self, token_type : &str)-> Token {

            //let current_token = self.next_token.clone(); // Clone next token to current local token
            let current_token = self.next_token.clone();
            if self.next_token.ttype == "".to_string() || self.next_token.ttype != token_type {
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
        fn program(&mut self) -> Node {
            return self.expression();
        }

        /**
         * Expression
         *   : Literal
         *   ;
         */
        fn expression(&mut self) -> Node {
            return self.literal();
            //let left = self.literal();
            
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
            let token = self.eat_token("NUMBER");
            /*
            return Node {
                ntype: String::from("NumericLiteral"),
                nvalue: token.tvalue
            }
            */
            Node::new_without_children("NumericLiteral", token.tvalue)
        }

        /**
         * StringLiteral
         *   : STRING
         */
        fn string_literal(&mut self) -> Node {
            let token = self.eat_token("STRING");
            Node::new_without_children("StringLiteral", token.tvalue)
        }
    }
}