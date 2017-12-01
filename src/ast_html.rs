use ast::{Content, Node};
use html::IntoHtml;
use parse::Event;

struct Context {}

impl Context {
    fn new() -> Context {
        Context {}
    }
}

impl<'a> IntoHtml<Context> for Event<'a> {
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        match self {
            &mut Event::Start(_) | &mut Event::End(_) => unreachable!(),
            &mut Event::Text(ref text) => buf.push_str(text),
            _ => panic!("AHHHHHHH!!!!!!!!!!"),
        }
    }
}

impl<'a> IntoHtml<Context> for Node<'a> {
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        match self {
            &mut Node::Block(ref tag, ref mut content) => {
                buf.push_str("<p>");
                content.render(context, buf);
                buf.push_str("</p>\n");
            },
            &mut Node::Item(ref mut event) => event.render(context, buf),
        }

    }
}

impl<'a, I> IntoHtml<Context> for Content<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        for mut node in self {
            node.render(context, buf);
        }
    }
}

pub fn into_html<'a, I>(content: &mut Content<'a, I>, buf: &mut String)
where
    I: Iterator<Item = Event<'a>>
{
    let mut context = Context::new();
    content.render(&mut context, buf);
}
