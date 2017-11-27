extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Node, Tag};

#[test]
fn ast_tag() {
    let original = r##"# Hello!
"##;

    let p = Parser::new(&original);
    let ast = Node::new(p).unwrap();

    assert_eq!(&Tag::Header(1), ast.tag());
}

#[test]
fn ast_contents_first_node() {
    let original = r##"# [link](/to/here)
"##;

    let p = Parser::new(&original);
    let mut ast = Node::new(p).unwrap();

    assert_eq!(&Tag::Header(1), ast.content.next().unwrap().tag());

}
