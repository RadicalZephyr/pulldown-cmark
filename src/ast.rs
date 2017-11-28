pub use parse::{Alignment, Event, Tag, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

use std::borrow::Borrow;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::mem::{discriminant,swap};
use std::rc::Rc;

#[derive(Default)]
struct KeepUntil<'a> {
    _t: PhantomData<Event<'a>>,
}

#[derive(Default)]
struct DropUntil<'a> {
    _t: PhantomData<Event<'a>>,
}


impl<'a> Iterator for KeepUntil<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        None
    }
}

impl<'a> Iterator for DropUntil<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        None
    }
}

fn split_when<'a, I, F>(iter: I, pred: F) -> (KeepUntil<'a>, DropUntil<'a>) {
    (KeepUntil::default(), DropUntil::default())
}

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
                    let tag2 = tag.clone();

                    let pred = move |e: &Event<'a>| {
                        match *e {
                            Event::End(ref end_tag) => discriminant(tag2.borrow()) == discriminant(end_tag),
                            _ => true,
                        }
                    };
                    let content = iter.take_while(pred);
                          let content = Content::new(Box::new(content));
                    Node {
                        tag,
                        content,
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
    iter: Option<Box<Iterator<Item = Event<'a>>>>,
}

impl<'a> Iterator for Content<'a>
where
    'a: 'static,
{
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        let mut iter = None;
        swap(&mut self.iter, &mut iter);
        iter.and_then(|i| Node::new(i))
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
            iter: Some(iter)
        }
    }
}
