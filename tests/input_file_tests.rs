use std::env;
use std::fs; // File system
use thymelang::parser;
use thymelang::interpreter;

fn run_interpreter_test(filename: &str, expected: &str) {
    let input_file_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //let file_path = input_file_dir.push_str("resources/test_files/simple.thm");
    let source = fs::read_to_string("./resources/test_files/".to_owned()+filename)
        .expect("Error: Input file not found!");
    let mut prs = parser::Parser::new(&source);
    let ast = prs.parse();
    let mut interpreter = interpreter::Interpreter::new();
    let result = interpreter.interpret(ast);
    assert_eq!(expected, result);   
}

#[test]
fn test_interpreter_with_input_files() {
    run_interpreter_test("test1.thm", "10");
    run_interpreter_test("test2.thm", "None");
}