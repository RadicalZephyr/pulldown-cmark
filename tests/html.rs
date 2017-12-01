// Tests for HTML spec.

extern crate pulldown_cmark;


#[cfg(unused)]
mod unused {
#[test]
fn html_test_1() {
    let original = r##"Little header

<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_2() {
    let original = r##"Little header

<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_3() {
    let original = r##"Little header

<?
<div></div>
<p>Useless</p>
?>"##;
    let expected = r##"<p>Little header</p>
<?
<div></div>
<p>Useless</p>
?>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_4() {
    let original = r##"Little header

<!--
<div></div>
<p>Useless</p>
-->"##;
    let expected = r##"<p>Little header</p>
<!--
<div></div>
<p>Useless</p>
-->"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_5() {
    let original = r##"Little header

<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;
    let expected = r##"<p>Little header</p>
<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_6() {
    let original = r##"Little header

<!X
Some things are here...
>"##;
    let expected = r##"<p>Little header</p>
<!X
Some things are here...
>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_7() {
    let original = r##"Little header
-----------

<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<h2>Little header</h2>
<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}
}
use pulldown_cmark::{Parser, html};

#[test]
fn renders_text() {
    let original = r##"Hello"##;

    let p = Parser::new(&original);
    let mut buf = String::new();
    html::push_html(&mut buf, p);

    assert_eq!("<p>Hello</p>\n", buf);
}

#[test]
fn renders_multiple_text() {
    let original = r##"Hello

World
"##;

    let p = Parser::new(&original);
    let mut buf = String::new();
    html::push_html(&mut buf, p);

    assert_eq!("<p>Hello</p>\n<p>World</p>\n", buf);

}


#[test]
fn renders_headings() {
    let original = r##"# Hello
"##;

    let p = Parser::new(&original);
    let mut buf = String::new();
    html::push_html(&mut buf, p);

    assert_eq!("<h1>Hello</h1>\n", buf);
}
