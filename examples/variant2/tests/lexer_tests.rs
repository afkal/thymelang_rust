use compiler_poc::lexer;

fn inspect_tokenvector(tokens: Vec<(String, String)>) -> String {
    let mut result = "".to_string();
    for (name, value) in tokens {
        result.push('(');
        result.push_str(&name);
        result.push(':');
        result.push_str(&value);
        result.push(')');
    }
    result
    //return "(NUM: 2)(NUM: 3)".to_string();
}
fn test_lexer(input: &str, expected: &str) {
    let raw_result = lexer::tokenize(input.to_string());
    let result = inspect_tokenvector(raw_result);
    assert_eq!(expected, result);    
}
#[test]
fn intdigit_plus_intdigit() {
    let input = "2+3";
    let expected = "(NUM:2)(OPER:PLUS)(NUM:3)";
    test_lexer(input, expected);
}

#[test]
fn intdigit_plus_intdigit_inparenthesis() {
    let input = "(2+3)";
    let expected = "(OPER:LPAR)(NUM:2)(OPER:PLUS)(NUM:3)(OPER:RPAR)";
    test_lexer(input, expected);
}
#[test]
fn int_times_int() {
    let input = "123*654";
    let expected = "(NUM:123)(OPER:MUL)(NUM:654)";
    test_lexer(input, expected);
}
#[test]
fn letter_plus_letter() {
    let input = "a+a";
    let expected = "(IDENTIFIER:a)(OPER:PLUS)(IDENTIFIER:a)";
    test_lexer(input, expected);
}
#[test]
fn word_plus_word_inparenthesis() {
    let input = "(abc+cde)";
    let expected = "(OPER:LPAR)(IDENTIFIER:abc)(OPER:PLUS)(IDENTIFIER:cde)(OPER:RPAR)";
    test_lexer(input, expected);
}
#[test]
fn identifier_with_line_should_be_ok() {
    let input = "abc_cde";
    let expected = "(IDENTIFIER:abc_cde)";
    test_lexer(input, expected);
}
#[test]
fn identifier_with_dash_should_be_ok() {
    let input = "abc-cde";
    let expected = "(IDENTIFIER:abc-cde)";
    test_lexer(input, expected);
}