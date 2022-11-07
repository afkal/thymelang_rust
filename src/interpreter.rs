use crate::parser::Node;


fn visit_binaryop(node: Node) -> String {

    let left = visit(node.children[0].clone());
    let right = visit(node.children[1].clone());
    let left_int = left.parse::<i32>().unwrap();
    let right_int = right.parse::<i32>().unwrap();   

    match node.nvalue.as_str() {
        "+" => return (left_int+right_int).to_string(),
        "-" => return (left_int-right_int).to_string(),
        "*" => return (left_int*right_int).to_string(),
        "/" => return (left_int/right_int).to_string(),
        _ => panic!("Invalid binary operator: \"{}\"", node.nvalue)
    }
    return node.nvalue.clone();
}

fn visit_unaryop(node: Node) -> String {

    /* In case unaryop is "+" return value of child node */
    if node.nvalue == "+" {
        return node.children[0].nvalue.clone();
    }

    match node.children[0].ntype.as_str() {
        "NumericLiteral" => return String::from("-")+&node.children[0].nvalue,
        _ => panic!("Unaryoperation not supported for type: \"{}\". Error occurred with item: {}", node.children[0].ntype, node.children[0].nvalue),
    }
}

/// Visit all children of Program Root node
fn visit_program(node: Node) -> String {

    let mut result = String::from("");
    for child in node.children {
        result = visit(child);
    }
    /* Currently only last result (=source code line) is returned */
    return result;
}

/// Entry point for interpreting program
fn visit(node: Node) -> String {

    match node.ntype.as_str() {
        "Program" => return visit_program(node),
        "NumericLiteral" | "StringLiteral" => return node.nvalue,
        "UnaryOp" => return visit_unaryop(node),
        "AdditiveExpression" | "MultiplicationTerm" => return visit_binaryop(node),
        _ => return String::from("Thyme Error: Could not interpret."),
    }
}

/// Interpret AST provided (by parser)
pub fn interpret(ast: Node) {
    let result = visit(ast);
    println!("{}", result)
}