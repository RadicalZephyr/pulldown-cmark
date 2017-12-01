use ast::{Content, Node};
use html::IntoHtml;
use parse::Event;

struct Context {}

impl Context {
    fn new() -> Context {
        Context {}
    }
}

impl<'a, I> IntoHtml<Context> for Content<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn render(&self, ctx: &mut Context, buf: &mut String) {
        buf.push_str("<p>Hello</p>");
    }
}

pub fn into_html<'a, I>(content: &mut Content<'a, I>, buf: &mut String)
where
    I: Iterator<Item = Event<'a>>
{
    let mut context = Context::new();
    content.render(&mut context, buf);
}
