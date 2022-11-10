use regex::Regex;

const REGEX_ARRAY: [(&str,&str);17] = [
    (r"\n", "EOL"), // Newline
    // Numbers
    (r"^[0-9]+\.[0-9]+", "FLOAT_NUMBER"), // eg. 123.4, needs to have decimal point and at least one decimal
    (r"^\d+", "INT_NUMBER"), // INT NEEDS TO BE AFTER FLOAT OR FLOAT WILL NOT KICK IN
    // Operators
    (r"^\(", "LPAREN"), // LEFT PARENTHESIS "("
    (r"^\)", "RPAREN"), // RIGHT PARENTHESIS ")"
    (r"^\*", "MUL"), // MULTIPLY operator "*"
    (r"^/", "DIV"), // DIVIDE operator "/"
    (r"^\+", "PLUS"), // PLUS operator "+"
    (r"^-", "MINUS"), // MINUS operator "-"
    (r"^==", "EQUAL"), // EQUALS operator "=="
    (r"^=", "ASSIGN"),  // ASSIGN operator "="
    (r"^;", "SEMICOLON"), // SEMICOLON ";"
    (r"^\{", "LCURLY"), // LEFT CURLY BRACKET "{"
    (r"^\}", "RCURLY"), // RIGHT CURLY BRACKET "}"
    // Reserved words
    (r"^print", "PRINT"), // PRINT statement
    // String
    (r####"^"(.*?)""####, "STRING"), // STRING
    //(re(stringFilter), "STRING"),
    // Identifier
    (r"^[_a-z][a-zA-Z0-9_]*", "IDENTIFIER") // everything that do not match reserved words
];

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


    pub fn get_next_token(&mut self) -> Token {

        // Quit if no more chars on source
        if self.position >= self.source.chars().count().try_into().unwrap() {
            // source consumed
            return Token::new("EOF", "EOF");
        }

        // Consume spaces
        if self.source.chars().nth(self.position).unwrap()==' ' {
            self.position += 1;
            if self.position >= self.source.chars().count().try_into().unwrap() {
                return Token::new("EOF", "EOF");
            }
        }

        // Target is the remaining source to evaluate
        let target = &self.source[self.position..self.source.len()];

        // Loop regex array to find matching token
        for item in REGEX_ARRAY {
            //println!("Evaluating regexp: {}", item.0);
            let re = Regex::new(item.0).unwrap();
            if re.is_match(&target) {
                let cap = re.find(&target).unwrap();
                //println!("Match found with regexp: {}",item.0);
                //println!("Match: {:#?}",cap);
                self.position += cap.end();
                return Token::new(item.1, cap.as_str());
            }
        }
        // No Match found - return error
        //return Token::new("ERROR", "ERROR");
        let size : usize;
        if target.len() < 15 {
            size = target.len();
        } else {
            size = 15;
        }
        panic!("Parser error: Unknown token \"{}\" at position: {}", &target[0..size], self.position);
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

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::lexer::Tokenizer;
    use crate::lexer::Token;

    #[test]
    fn test_tokenizer_single_integer() {
        let mut tokenizer = Tokenizer::new("153");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("INT_NUMBER"),
            tvalue: String::from("153")
        };
        assert_eq!(expected, result);
    }
    #[test]
    fn test_tokenizer_single_integer_with_spaces() {
        let mut tokenizer = Tokenizer::new("      153");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("INT_NUMBER"),
            tvalue: String::from("153")
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenizer_single_float() {
        let mut tokenizer = Tokenizer::new("153.23423432");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("FLOAT_NUMBER"),
            tvalue: String::from("153.23423432")
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenizer_invalid_float() {
        let mut tokenizer = Tokenizer::new("153.234.23432");
        let result = tokenizer.get_next_token();
        let nexpected = Token {
            ttype: String::from("FLOAT_NUMBER"),
            tvalue: String::from("153.234")
        };
        assert_eq!(nexpected, result);
    }

    #[test]
    fn test_tokenizer_short_expression() {
        let mut tokenizer = Tokenizer::new("15+3");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("INT_NUMBER"),
            tvalue: String::from("15")
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenizer_single_string() {
        let mut tokenizer = Tokenizer::new("\"jiihaa\"");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("STRING"),
            tvalue: String::from("\"jiihaa\"")
        };
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_tokenizer_incomplete_string() {
        let mut tokenizer = Tokenizer::new("\"jiihaa");
        let result = tokenizer.get_next_token();
        let nexpected = Token {
            ttype: String::from("ERROR"),
            tvalue: String::from("ERROR")
        };
        assert_eq!(nexpected, result);
    }

    #[test]
    fn test_valid_identifier() {
        let mut tokenizer = Tokenizer::new("jii_haa16");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("IDENTIFIER"),
            tvalue: String::from("jii_haa16")
        };
        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_invalid_identifier() {
        let mut tokenizer = Tokenizer::new("Jii_haa16");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("ERROR"),
            tvalue: String::from("ERROR")
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_short_identifier() {
        let mut tokenizer = Tokenizer::new("a");
        let result = tokenizer.get_next_token();
        let expected = Token {
            ttype: String::from("IDENTIFIER"),
            tvalue: String::from("a")
        };
        assert_eq!(expected, result);
    }

}