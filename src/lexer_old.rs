pub mod lexer {

    #[derive(PartialEq, Debug, Clone)]
    pub struct Token {
        pub ttype: String,
        pub tvalue: String
    }

    impl Token {
        pub fn new(ttype: &str, tvalue: &str) -> Self {
            Self {
                ttype: String::from(ttype),
                tvalue: String::from(tvalue)
            }
        }
    }
    pub struct Tokenizer {
        source: String,
        position: usize
    }
    
    impl Tokenizer {
    
        pub fn new(source: &str) -> Tokenizer {
            //let mut tokenizer = Tokenizer {
            let tokenizer = Tokenizer {
                source: String::from(source).trim().to_string(),
                position: 0
            };
            tokenizer
        }
    
        fn get_current_char(&self) -> Option<char> {
            if self.position >= self.source.chars().count().try_into().unwrap() {
                return None;
            }
            let c = self.source.chars().nth(self.position).unwrap();
            return Some(c);
        }
    
    
        fn advance_position(&mut self) {
            self.position += 1;
        }
    
        /* Get next token */
        pub fn get_next_token(&mut self) -> Token {

            // Quit if no more chars on source
            if self.position >= self.source.chars().count().try_into().unwrap() {
                // source consumed
                return Token::new("EOF", "EOF");
            }
    
            let mut current_char = self.get_current_char().unwrap();

            // Numbers: \d+
            //let filter = "^\d+";
            if current_char.is_digit(10) {
                let mut number = String::from("");
                let mut dot_count = 0;
                while current_char.is_digit(10) || current_char == '.' {
                    if current_char == '.' {
                        dot_count +=1;
                    }
                    if dot_count > 1 {
                        panic!("Invalid float with {} dots", dot_count);
                    }
                    number.push(current_char);
                    self.advance_position();
                    match self.get_current_char() {
                        None => return Token::new("NUMBER", &number),                      
                        Some(c) => current_char = c // get next char and continue collectin number
                    }
                }
                return Token::new("NUMBER", &number);
            }

            // String:
            if current_char == '"' {
                let mut s = "".to_string();
                loop {
                    self.advance_position();
                    match self.get_current_char() {
                        None => panic!("Invalid string missing ending quotation mark at: {}", self.position), // end of source, without ending quatition mark
                        Some(c) => current_char = c // get next char and continue collectin number

                    }
                    //println!("Processing: {}", current_char);
                    if current_char == '"' {
                        self.advance_position(); // skip ending quatition mark
                        break; // break loop if ending quatition mark found
                    }
                    s.push(current_char); // build string char by char
                }
                return Token {
                    ttype: String::from("STRING"),
                    tvalue: s
                }
            }

            // +
            if current_char == '+' {
                self.advance_position();
                return Token {
                    ttype: String::from("PLUS"),
                    tvalue: String::from("+"),
                }
            }
            // No token found
            return Token {
                ttype: String::from("ERROR"),
                tvalue: String::from("ERROR"),
            }
        }

        pub fn print_all_tokens(&mut self) {
            /*
             * Read and print all the tokens from source one by one
             */
            loop {
                let token = self.get_next_token();
                println!("{} {} at position: {}",token.ttype, token.tvalue, self.position);
                if token.ttype == "EOF" ||token.ttype == "ERROR" {
                    break;
                }
            }
        }
    
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::lexer::lexer as lxr;
    use crate::lxr::Token;

    #[test]
    fn test_tokenizer_single_integer() {
        let mut tokenizer = lxr::Tokenizer::new("153");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("NUMBER"),
            tvalue: String::from("153")
        };
        assert_eq!(expected, result);
    }
    #[test]
    fn test_tokenizer_single_integer_with_spaces() {
        let mut tokenizer = lxr::Tokenizer::new("      153");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("NUMBER"),
            tvalue: String::from("153")
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenizer_single_float() {
        let mut tokenizer = lxr::Tokenizer::new("153.23423432");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("NUMBER"),
            tvalue: String::from("153.23423432")
        };
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_tokenizer_invalid_float() {
        let mut tokenizer = lxr::Tokenizer::new("153.234.23432");
        let result = tokenizer.get_next_token();
        let nexpected = Token {
            ttype: String::from("NUMBER"),
            tvalue: String::from("153.234.23432")
        };
        assert_ne!(nexpected, result);
    }

    #[test]
    fn test_tokenizer_short_expression() {
        let mut tokenizer = lxr::Tokenizer::new("15+3");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("NUMBER"),
            tvalue: String::from("15")
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenizer_single_string() {
        let mut tokenizer = lxr::Tokenizer::new("\"jiihaa\"");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("STRING"),
            tvalue: String::from("jiihaa")
        };
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_tokenizer_incomplete_string() {
        let mut tokenizer = lxr::Tokenizer::new("\"jiihaa");
        let result = tokenizer.get_next_token();
        let nexpected = Token {
            ttype: String::from("STRING"),
            tvalue: String::from("jiihaa")
        };
        assert_ne!(nexpected, result);
    }
}