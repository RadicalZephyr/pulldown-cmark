extern crate pulldown_cmark;

use std::fmt::Debug;

use pulldown_cmark::{Content, Parser, Event, Tag, Node};

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
    let mut content = Content::new(p);
    let content_head = content.next();
    assert!(content_head.is_some());
    match content_head {
        Some(Node::Block(tag, _)) =>
            assert_eq!(Tag::Header(1), tag),
        _ => assert!(false)
    }


}

#[test]
fn first_child_of_first_node() {
    let original = r##"# [link](/to/here)
    "##;

    let p = Parser::new(&original);
    let v: Vec<_>  = p.collect();
    print_iter(v.iter());
    println!("\n\n");

    let mut content = Content::new(v.into_iter());

    match content.next() {
        Some(Node::Block(_, mut content)) => {
            println!("{:?}", content);
            match content.next() {
                Some(Node::Block(first_child, _)) => {

                    assert_eq!(Tag::Link("/to/here".into(), "".into()), first_child)
                },
                Some(Node::Item(e)) => assert_eq!(Event::SoftBreak, e),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn ast_contents_second_node() {
    let original = r##"# [link](/to/here)

Hello
"##;

    let p = Parser::new(&original);
    let v: Vec<_>  = p.collect();
    print_iter(v.iter());
    println!("\n\n");

    let mut content = Content::new(v.into_iter());
    println!("{:?}\n\n", content);
    let content_head = {
        content.next(); // Skip the first Node
        println!("{:?}\n\n", content);
        content.next()
    };
    println!("{:?}\n\n", content);
    assert!(content_head.is_some());
    match content_head {
        Some(Node::Block(tag, _)) =>
            assert_eq!(Tag::Paragraph, tag),
        _ => assert!(false)
    }


}
