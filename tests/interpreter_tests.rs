use thymelang_rust::parser;
use thymelang_rust::parser::Node;
use thymelang_rust::interpreter::{interpret};

fn run_interpreter_test(input: &str, expected: &str) {
    let mut prs = parser::Parser::new(input);
    let ast = prs.parse();
    let result = interpret(ast);
    assert_eq!(expected, result);   
}



#[test]
fn test_additive_expression() {
    let mut prs = parser::Parser::new("1+2");
    let ast = prs.parse();
    let result = interpret(ast);
    let expected = "3";
    assert_eq!(expected, result);
}

#[test]
fn test_calculator_expressions() {
    run_interpreter_test("1+2", "3");
    run_interpreter_test("2*3", "6");
    run_interpreter_test("2*(3+4)", "14");
    run_interpreter_test("7 + (((3 + 2)))", "12");
    run_interpreter_test("7 + 3 * (10 / (12 / (3 + 1) - 1))", "22");
    run_interpreter_test("7 + 3 * (10 / (12 / (3 + 1) - 1)) / (2 + 3) - 5 - 3 + (8)", "10");
}