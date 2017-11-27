extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Node, Tag};

#[test]
fn ast_test_1() {
    let original = r##"# Hello!
"##;

    let p = Parser::new(&original);
    let ast = Node::new(p).unwrap();

    assert_eq!(&Tag::Header(1), ast.tag());
}
