use thymelang::parser;
use thymelang::parser::Node;

#[test]
fn test_single_integer() {
    let mut prs = parser::Parser::new("1");
    let result = &prs.parse().children[0];
    let expected = Node::new_without_children("Integer", "1");
    assert_eq!(&expected, result);
}

#[test]
fn test_additive_expression() {
    let mut prs = parser::Parser::new("1+2");
    let result = &prs.parse().children[0];
    let left = Node::new_without_children("Integer", "1");
    let right = Node::new_without_children("Integer", "2");
    let children = Vec::from([left, right]);
    let expected = Node::new("AdditiveExpression", "+", children);
    /*
    let expected = Node {
        ntype: String::from("AdditiveExpression"),
        nvalue: String::from("+"),
        children: Vec::from([left,right])
    };
    */
    assert_eq!(&expected, result);
}

#[test]
fn test_multiplication_term() {
    let mut prs = parser::Parser::new("1*2");
    let result = &prs.parse().children[0];
    let left = Node::new_without_children("Integer", "1");
    let right = Node::new_without_children("Integer", "2");
    let children = Vec::from([left, right]);
    let expected = Node::new("MultiplicationTerm", "*", children);
    assert_eq!(&expected, result);
}

#[test]
fn test_parenthesis() {
    let mut prs = parser::Parser::new("(1+2)");
    let result = &prs.parse().children[0];
    let left = Node::new_without_children("Integer", "1");
    let right = Node::new_without_children("Integer", "2");
    let children = Vec::from([left, right]);
    let expected = Node::new("AdditiveExpression", "+", children);
    assert_eq!(&expected, result);
}

#[test]
fn test_order_of_operation() {
    let mut prs = parser::Parser::new("1+2*3");
    let result = &prs.parse().children[0];
    let result_json = serde_json::to_string(&result).unwrap();
    let expexted = "{\"ntype\":\"AdditiveExpression\",\"nvalue\":\"+\",\"children\":[{\"ntype\":\"Integer\",\"nvalue\":\"1\",\"children\":[]},{\"ntype\":\"MultiplicationTerm\",\"nvalue\":\"*\",\"children\":[{\"ntype\":\"Integer\",\"nvalue\":\"2\",\"children\":[]},{\"ntype\":\"Integer\",\"nvalue\":\"3\",\"children\":[]}]}]}";
    assert_eq!(expexted, result_json);
}

#[test]
fn test_parenthesis_order_of_operation() {
    let mut prs = parser::Parser::new("(1+2)*3");
    let result = &prs.parse().children[0];
    let result_json = serde_json::to_string(&result).unwrap();
    let expexted = "{\"ntype\":\"MultiplicationTerm\",\"nvalue\":\"*\",\"children\":[{\"ntype\":\"AdditiveExpression\",\"nvalue\":\"+\",\"children\":[{\"ntype\":\"Integer\",\"nvalue\":\"1\",\"children\":[]},{\"ntype\":\"Integer\",\"nvalue\":\"2\",\"children\":[]}]},{\"ntype\":\"Integer\",\"nvalue\":\"3\",\"children\":[]}]}";
    assert_eq!(expexted, result_json);
}

#[test]
fn test_unary_operator() {
    let mut prs = parser::Parser::new("1+-2");
    let result = &prs.parse().children[0];
    let result_json = serde_json::to_string(&result).unwrap();
    let expexted = "{\"ntype\":\"AdditiveExpression\",\"nvalue\":\"+\",\"children\":[{\"ntype\":\"Integer\",\"nvalue\":\"1\",\"children\":[]},{\"ntype\":\"UnaryOp\",\"nvalue\":\"-\",\"children\":[{\"ntype\":\"Integer\",\"nvalue\":\"2\",\"children\":[]}]}]}";
    assert_eq!(expexted, result_json);
}

#[test]
fn test_simple_inline_comment() {
    let mut prs = parser::Parser::new("1; // comment");
    let result = &prs.parse().children[0];
    let expected = Node::new_without_children("Integer", "1");
    assert_eq!(&expected, result);
}