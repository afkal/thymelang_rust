// Struct for single Token (currently not used)
/*
struct Token{
    name: String,
    value: String
}
*/
fn parse_operator(ch: char) -> String {
    if ch == '+' {
        return "ADD".to_string();
    }
    if ch == '-' {
        return "SUB".to_string();
    }
    if ch == '*' {
        return "MUL".to_string();
    }
    if ch == '/' {
        return "DIV".to_string();
    }
    if ch == '(' {
        return "LPAR".to_string();
    }
    if ch == ')' {
        return "RPAR".to_string();
    }
    return "invalid".to_string();
}

pub fn tokenize(line: &mut String) -> Vec<(String, String)> {

    // Add extra char to line in order to make sure whole line is covered.
    line.push(' ');
    let mut tokens: Vec<(String, String)> = Vec::new();
    let mut iterator = line.chars();
    //let mut ch;
    while let Some(ch) = iterator.next() {
        // Found comment - skip rest of the line
        if ch == '#' {
            break;
        }
        // Found digit -> process number
        if ch.is_digit(10) {
            let mut number = "".to_string();
            number.push(ch);
            let mut punct_count = 0;
            // Evaluating next characters
            while let Some(ch) = iterator.next() {
                if ch.is_digit(10) {
                    number.push(ch);
                    continue;
                }
                if ch=='.' && punct_count == 0 {
                    punct_count += 1;
                    number.push(ch);
                    continue;
                }
                if ch.is_whitespace() {
                    // Number complete push to tokens
                    tokens.push(("NUM".to_string(), number));
                    break; // break digit loop when whitespace is found
                }
                if ch.is_alphabetic() {
                    number.push(ch);
                    println!("ERROR - invalid identifier: {}", number);
                    break;
                }
                // Special character, store number and sort out the operator
                tokens.push(("NUM".to_string(), number));
                tokens.push(("OPER".to_string(), parse_operator(ch)));
                break;
            }
            continue; // get back to start with new identifier
        }
        // Found alphabet -> process identifier
        if ch.is_alphabetic() {
            let mut identifier = "".to_string();
            identifier.push(ch);
            while let Some(ch) = iterator.next() {
                if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                    identifier.push(ch);
                    continue;
                }
                if ch.is_whitespace() {
                    // Identifier complete push to tokens
                    tokens.push(("IDENTIFIER".to_string(), identifier));
                    break; // break digit loop when whitespace is found
                }
                // DEFAULT - none of the above, store identifier and sort out the operator
                tokens.push(("IDENTIFIER".to_string(), identifier));
                // TODO: Handle unknown operator
                tokens.push(("OPER".to_string(), parse_operator(ch)));
                break;            
            }
            continue; // get back to start with new identifier
        }
        // Found space - skip
        if ch.is_whitespace() {
            continue;
        }
        // Handle special character
        tokens.push(("OPER".to_string(), parse_operator(ch)));
        continue;
    }
    
    tokens

}