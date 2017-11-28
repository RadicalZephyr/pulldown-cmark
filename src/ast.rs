pub use parse::{Alignment, Event, Tag, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

use std::borrow::Borrow;
use std::default::Default;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::mem::{discriminant,swap};
use std::rc::Rc;

#[derive(Default)]
struct KeepUntil<'a, 'b, I, F>
where
    'a: 'b,
    I: 'b + Iterator<Item = Event<'a>>
{
    iter: Option<Rc<I>>,
    pred: Rc<F>,
    _t: PhantomData<Event<'a>>,
    _t2: PhantomData<&'b Iterator<Item = Event<'a>>>,
}

#[derive(Default)]
struct DropUntil<'a, 'b, I, F>
where
    'a: 'b,
    I: 'b + Iterator<Item = Event<'a>>
{
    iter: Option<Rc<I>>,
    pred: Rc<F>,
    _t: PhantomData<Event<'a>>,
    _t2: PhantomData<&'b Iterator<Item = Event<'a>>>,
}


impl<'a, 'b, I, F> Iterator for KeepUntil<'a, 'b, I, F>
where
    'a: 'b,
    I: 'b + Iterator<Item = Event<'a>>
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        None
    }
}

impl<'a, 'b, I, F> Iterator for DropUntil<'a, 'b, I, F>
where
    I: 'b + Iterator<Item = Event<'a>>
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        None
    }
}

fn split_when<'a, 'b, I, F>(iter: I, pred: F) -> (KeepUntil<'a, 'b, I, F>, DropUntil<'a, 'b, I, F>)
where
    I: 'b + Iterator<Item = Event<'a>>
{
    let iter: Option<Rc<I>> = Rc::new(iter).into();
    let pred = Rc::new(pred);
    (KeepUntil { iter: iter.clone(), pred: pred.clone(),
                 _t: Default::default(), _t2: Default::default() },
     DropUntil { iter, pred,
                 _t: Default::default(), _t2: Default::default() })
}

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
        InIter: 'b + Iterator<Item = Event<'a>>
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
                    let (content, _rest) = split_when(iter, pred);
                    let content = Content::new(Box::new(content));
                    (Node {
                        tag,
                        content,
                    }, 0)
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
