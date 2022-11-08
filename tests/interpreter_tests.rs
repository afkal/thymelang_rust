use thymelang::parser;
use thymelang::interpreter::{interpret};

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

#[test]
fn test_unary_operator() {
    run_interpreter_test("-1", "-1");
    run_interpreter_test("--1", "1");
    run_interpreter_test("+(2*4)","8");
    run_interpreter_test("-(2*3)","-6");
    run_interpreter_test("-(2+3)","-5");
    run_interpreter_test("--+-+-8","8");
}

#[test]
fn test_multiple_expressions() {
    run_interpreter_test("1+2; 3*4;", "12");
}

#[test]
fn test_assignment_statements() {
    run_interpreter_test("a=2", "2");
    run_interpreter_test("a=10; b=a+5", "15");
    run_interpreter_test("a=10; b=a+5; c=5*a - - b;", "65");
}