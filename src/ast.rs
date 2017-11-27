pub use parse::{Alignment, Event, Tag, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

use std::borrow::Borrow;
use std::iter::Iterator;
use std::mem::discriminant;
use std::rc::Rc;

pub struct Node<'a> {
    tag: Rc<Tag<'a>>,
    pub content: Content<'a>,
}

impl<'a> Node<'a> {
    pub fn new<InIter>(mut iter: InIter) -> Option<Node<'a>>
        where
        'a: 'static,
        InIter: 'static + Iterator<Item = Event<'a>>
    {
        iter.next().map(|i| {
            match i {
                Event::Start(tag) => {
                    let tag = Rc::new(tag);
                    Node {
                        tag: tag.clone(),
                        content: Content::new(Box::new(iter.take_while(move |i| match *i {
                            Event::End(ref end_tag) => discriminant(tag.borrow()) == discriminant(end_tag),
                            _ => true,
                        }))),
                    }
                },
                _ => panic!(),
            }
        })
    }

    pub fn tag(&self) -> &Tag<'a> {
        self.tag.borrow()
    }
}

pub struct Content<'a> {
    iter: Box<Iterator<Item = Event<'a>>>,
}

impl<'a> Iterator for Content<'a>
where
    'a: 'static,
{
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        None
    }
}

impl<'a> Content<'a>
where
    'a: 'static,
{
    pub fn new<I>(iter: Box<I>) -> Content<'a>
    where
        I : 'static + Iterator<Item = Event<'a>>
    {
        Content {
            iter,
        }
    }
}
