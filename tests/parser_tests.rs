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
    /*
    let expected = Node {
        ntype: String::from("AdditiveExpression"),
        nvalue: String::from("+"),
        children: Vec::from([left,right])
    };
    */
    assert_eq!(expected, result);
}