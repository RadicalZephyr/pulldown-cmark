extern crate pulldown_cmark;

use std::fmt::Debug;

use pulldown_cmark::{Content, Parser, Tag};

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
    let mut content = Content::new(Box::new(p));
    let content_head = content.next();
    assert!(content_head.is_some());
    assert_eq!(&Tag::Header(1),
               content_head.unwrap().tag());

}

#[test]
fn ast_contents_first_node_first_child() {
    let original = r##"# [link](/to/here)
"##;

    let p = Parser::new(&original);

    let mut content = Content::new(Box::new(p));
    let content_head = content.next();

    let mut children = content_head.unwrap().content;
    let children_head = children.next();

    assert!(children_head.is_some());
    assert_eq!(&Tag::Link("/to/here".into(), "".into()),
               children_head.unwrap().tag());

}
