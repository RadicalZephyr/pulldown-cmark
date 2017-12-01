use ast::{Content, Node};
use html::IntoHtml;
use parse::{Event, Tag};

struct Context {}

impl Context {
    fn new() -> Context {
        Context {}
    }
}

impl<'a> IntoHtml<Context> for Tag<'a> {
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        match *self {
            Tag::Paragraph => buf.push('p'),
            Tag::Header(n) => { buf.push('h'); buf.push_str(&format!("{}", n))},
            _ => (),
        }
    }
}

impl<'a> IntoHtml<Context> for Event<'a> {
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        match *self {
            Event::Start(_) | Event::End(_) => unreachable!(),
            Event::Text(ref text) => buf.push_str(text),
            _ => panic!("AHHHHHHH!!!!!!!!!!"),
        }
    }
}

impl<'a> IntoHtml<Context> for Node<'a> {
    fn render(&mut self, context: &mut Context, buf: &mut String) {
        match *self {
            Node::Block(ref mut tag, ref mut content) => {
                buf.push('<');
                tag.render(context, buf);
                buf.push('>');

                content.render(context, buf);

                buf.push_str("</");
                tag.render(context, buf);
                buf.push_str(">\n")
            },
            Node::Item(ref mut event) => event.render(context, buf),
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
