extern crate pulldown_cmark;

use std::fmt::Debug;

use pulldown_cmark::{Content, Parser, Tag, Node};

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

// // #[test]
// fn first_child_of_first_node() {
//     let original = r##"# [link](/to/here)
// "##;

//     let p = Parser::new(&original);

//     let mut content = Content::new(Box::new(p));
//     let content_head = content.next();

//     let mut children = content_head.unwrap().content;
//     let children_head = children.next();

//     assert!(children_head.is_some());
//     assert_eq!(&Tag::Link("/to/here".into(), "".into()),
//                children_head.unwrap().tag());

// }

// // #[test]
// fn past_first_child() {
//     let original = r##"# [link](/to/here)
// "##;

//     let p = Parser::new(&original);
//     let mut content = Content::new(Box::new(p));
//     let head = content.next();
//     assert!(head.is_some());

//     let second = content.next();
//     assert!(second.is_some());

//     // assert_eq!(&Tag::Link("/to/here".into(), "".into()),
//     //            content_head.unwrap().tag());

// }
