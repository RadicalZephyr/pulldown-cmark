extern crate pulldown_cmark;

use pulldown_cmark::{into_html, IntoHtml, Parser};

#[test]
fn renders_text() {
    let original = r##"Hello"##;

    let p = Parser::new(&original);
    let mut content = p.into_ast();
    let mut buf = String::new();
    into_html(&mut content, &mut buf);
    assert_eq!("<p>Hello</p>", buf);
}
