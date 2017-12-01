extern crate pulldown_cmark;

use pulldown_cmark::{into_html, IntoHtml, Parser};

#[test]
fn renders_text() {
    let original = r##"Hello"##;

    let p = Parser::new(&original);
    let mut content = p.into_ast();
    let mut buf = String::new();
    into_html(&mut content, &mut buf);
    assert_eq!("<p>Hello</p>\n", buf);
}

#[test]
fn renders_multiple_text() {
    let original = r##"Hello

World
"##;

    let p = Parser::new(&original);
    let mut content = p.into_ast();
    let mut buf = String::new();
    into_html(&mut content, &mut buf);
    assert_eq!("<p>Hello</p>\n<p>World</p>\n", buf);
}

#[test]
fn renders_headings() {
    let original = r##"# Hello
"##;

    let p = Parser::new(&original);
    for i in p {
        println!("{:?}", i);
    }
    println!("\n\n");

    let p = Parser::new(&original);
    let mut content = p.into_ast();
    let mut buf = String::new();
    into_html(&mut content, &mut buf);
    assert_eq!("<h1>Hello</h1>\n", buf);
}

#[test]
fn renders_h2_headings() {
    let original = r##"## Hello
"##;

    let p = Parser::new(&original);
    for i in p {
        println!("{:?}", i);
    }
    println!("\n\n");

    let p = Parser::new(&original);
    let mut content = p.into_ast();
    let mut buf = String::new();
    into_html(&mut content, &mut buf);
    assert_eq!("<h2>Hello</h2>\n", buf);
}
