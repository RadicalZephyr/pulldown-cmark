extern crate pulldown_cmark;

use std::fmt::Debug;

use pulldown_cmark::{Parser, Node, Tag};

#[test]
fn ast_tag() {
    let original = r##"# Hello!
"##;

    let p = Parser::new(&original);
    let ast = Node::new(p).unwrap();

    assert_eq!(&Tag::Header(1), ast.tag());
}

fn print_iter<I, D>(iter: I)
    where
    D: Debug,
    I: Iterator<Item = D>
{
    for i in iter {
        println!("{:?}", i);
    }
}

#[test]
fn ast_contents_first_node() {
    let original = r##"# [link](/to/here)
"##;

    let p = Parser::new(&original);
    let mut ast = Node::new(p).unwrap();
    let content_head = ast.content.next();
    assert!(content_head.is_some());
    assert_eq!(&Tag::Link("/to/here".into(), "".into()),
               content_head.unwrap().tag());

}
