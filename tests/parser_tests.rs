use thymelang_rust::parser;
use thymelang_rust::parser::Node;

#[test]
fn test_additive_expression() {
    let mut prs = parser::Parser::new("1+2");
    let result = prs.parse();
    let left = Node::new_without_children("NumericLiteral", "1");
    let right = Node::new_without_children("NumericLiteral", "2");
    let children = Vec::from([left, right]);
    let expected = Node::new("AdditiveExpression", "+", children);
    /*
    let expected = Node {
        ntype: String::from("AdditiveExpression"),
        nvalue: String::from("+"),
        children: Vec::from([left,right])
    };
    */
    assert_eq!(expected, result);
}

#[test]
fn test_multiplication_term() {
    let mut prs = parser::Parser::new("1*2");
    let result = prs.parse();
    let left = Node::new_without_children("NumericLiteral", "1");
    let right = Node::new_without_children("NumericLiteral", "2");
    let children = Vec::from([left, right]);
    let expected = Node::new("MultiplicationTerm", "*", children);
    assert_eq!(expected, result);
}

#[test]
fn test_parenthesis() {
    let mut prs = parser::Parser::new("(1+2)");
    let result = prs.parse();
    let left = Node::new_without_children("NumericLiteral", "1");
    let right = Node::new_without_children("NumericLiteral", "2");
    let children = Vec::from([left, right]);
    let expected = Node::new("AdditiveExpression", "+", children);
    assert_eq!(expected, result);
}

#[test]
fn test_order_of_operation() {
    let mut prs = parser::Parser::new("1+2*3");
    let result = prs.parse();
    let result_json = serde_json::to_string(&result).unwrap();
    let expexted = "{\"ntype\":\"AdditiveExpression\",\"nvalue\":\"+\",\"children\":[{\"ntype\":\"NumericLiteral\",\"nvalue\":\"1\",\"children\":[]},{\"ntype\":\"MultiplicationTerm\",\"nvalue\":\"*\",\"children\":[{\"ntype\":\"NumericLiteral\",\"nvalue\":\"2\",\"children\":[]},{\"ntype\":\"NumericLiteral\",\"nvalue\":\"3\",\"children\":[]}]}]}";
    assert_eq!(expexted, result_json);
}

#[test]
fn test_parenthesis_order_of_operation() {
    let mut prs = parser::Parser::new("(1+2)*3");
    let result = prs.parse();
    let result_json = serde_json::to_string(&result).unwrap();
    let expexted = "{\"ntype\":\"MultiplicationTerm\",\"nvalue\":\"*\",\"children\":[{\"ntype\":\"AdditiveExpression\",\"nvalue\":\"+\",\"children\":[{\"ntype\":\"NumericLiteral\",\"nvalue\":\"1\",\"children\":[]},{\"ntype\":\"NumericLiteral\",\"nvalue\":\"2\",\"children\":[]}]},{\"ntype\":\"NumericLiteral\",\"nvalue\":\"3\",\"children\":[]}]}";
    assert_eq!(expexted, result_json);
}

#[test]
fn test_unary_operator() {
    let mut prs = parser::Parser::new("1+-2");
    let result = prs.parse();
    let result_json = serde_json::to_string(&result).unwrap();
    let expexted = "{\"ntype\":\"AdditiveExpression\",\"nvalue\":\"+\",\"children\":[{\"ntype\":\"NumericLiteral\",\"nvalue\":\"1\",\"children\":[]},{\"ntype\":\"UnaryOp\",\"nvalue\":\"-\",\"children\":[{\"ntype\":\"NumericLiteral\",\"nvalue\":\"2\",\"children\":[]}]}]}";
    assert_eq!(expexted, result_json);
}