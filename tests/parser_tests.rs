use thymelang_rust::parser;

#[test]
fn test_parse_single_integer() {
    let mut prs = parser::Parser::new("1123");
    let result = prs.parse();
    let expected = Node::new_without_children("NumericLiteral", "1123");
    assert_eq!(expected, result);
}