pub use parse::{Alignment, Event, Tag, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

use std::borrow::Borrow;
use std::default::Default;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::mem::{discriminant,swap};
use std::rc::Rc;

use super::split_by::SplitBy;

pub struct Node<'a, 'b>
where
    'a: 'b,
{
    tag: Rc<Tag<'a>>,
    pub content: Content<'a, 'b>,
}

impl<'a, 'b> Node<'a, 'b>
where
    'a: 'b,
{
    pub fn new<InIter>(mut iter: InIter) -> Option<(Node<'a, 'b>, u8)>
    where
        InIter: 'b + Iterator<Item = Event<'a>>,
    {
        iter.next().map(|i| {
            match i {
                Event::Start(tag) => {
                    let tag = Rc::new(tag);
                    let tag2 = tag.clone();

                    let pred = move |e: &Event<'a>| {
                        match *e {
                            Event::End(ref end_tag) => discriminant(tag2.borrow()) == discriminant(end_tag),
                            _ => true,
                        }
                    };
                    let (content, _rest) = iter.split_by(pred);
                    let content = Content::new(Box::new(content));
                    (Node {
                        tag,
                        content,
                    },
                    0)
                },
                _ => panic!(),
            }
        })
    }

    pub fn tag(&self) -> &Tag<'a> {
        self.tag.borrow()
    }
}

pub struct Content<'a, 'b>
    where
    'a: 'b,
{
    iter: Option<Box<'b + Iterator<Item = Event<'a>>>>,
}

impl<'a, 'b> Iterator for Content<'a, 'b>
where
    'a: 'b,
{
    type Item = Node<'a, 'b>;

    fn next(&mut self) -> Option<Node<'a, 'b>> {
        let mut iter = None;
        swap(&mut self.iter, &mut iter);
        iter.and_then(|i| {
            Node::new(i).map(|(node, _)| node)
        })
    }
}

impl<'a, 'b> Content<'a, 'b>
where
    'a: 'b,
{
    pub fn new<I>(iter: Box<I>) -> Content<'a, 'b>
    where
         I : 'b + Iterator<Item = Event<'a>>
    {
        Content {
            iter: Some(iter)
        }
    }
}
