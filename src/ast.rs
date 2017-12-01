pub use parse::{Alignment, Event, Tag, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

use std::iter::{Iterator, Peekable};
use std::marker::PhantomData;
use std::mem::discriminant;
use std::vec::IntoIter;

use super::collect_while;

#[derive(Debug)]
pub enum Node<'a> {
    Block(Tag<'a>, Content<'a, IntoIter<Event<'a>>>),
    Item(Event<'a>),
}

impl<'a> Node<'a> {
    pub fn try_from<I>(iter: &mut Peekable<I>) -> Option<Node<'a>>
    where
        I: Iterator<Item = Event<'a>>,
    {
        match iter.next() {
            Some(Event::Start(start_tag)) => {
                let content: Vec<_> = collect_while(iter, |event| {
                    match *event {
                        Event::End(ref end_tag) =>
                            discriminant(&start_tag) != discriminant(&end_tag),
                        _ => true,
                    }
                });
                Node::Block(
                        start_tag,
                        Content::new(content.into_iter()),
                ).into()
            },
            Some(Event::End(_)) => Node::try_from(iter),
            Some(event) => Some(Node::Item(event)),
            None => None
        }
    }
}

#[derive(Debug)]
pub struct Content<'a, I>
where I: Iterator<Item = Event<'a>> {
    iter: Peekable<I>,
    _t: PhantomData<&'a str>,
}

impl<'a, I> Content<'a, I>
where I: Iterator<Item = Event<'a>> {
    pub fn new(iter: I) -> Content<'a, I> {
        Content {
            iter: iter.peekable(),
            _t: PhantomData,
        }
    }
}

impl<'a, I> Iterator for Content<'a, I>
where I: Iterator<Item = Event<'a>> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        Node::try_from(&mut self.iter)
    }
}

// pub struct Node<'a, I, T>
// where
//     I: Iterator<Item = Event<'a>>
// {
//     tag: T,
//     pub content: Content<'a, I>
// }

// impl<'a, I, T> Node<'a, I, T>
// where
//     I: Iterator<Item = Event<'a>>
// {
//     pub fn try_from(iter: I) -> Option<(Node<'a, I, T>, I)>
//     {
//         iter.next().map(|i| {
//             match i {
//                 Event::Start(tag) => {
//                     let tag: Rc<Tag<'a>> = Rc::new(tag);
//                     let tag2 = tag.clone();

//                     let pred = move |e: &Event<'a>| {
//                         match *e {
//                             Event::End(ref end_tag) => discriminant(tag2.borrow()) == discriminant(end_tag),
//                             _ => true,
//                         }
//                     };
//                     let (content, rest) = split_by(iter, pred);
//                     let content = Content::new(Box::new(content));
//                     (Node {
//                         tag,
//                         content,
//                     },
//                      rest)
//                 },
//                 _ => panic!(),
//             }
//         })
//     }

//     pub fn tag(&self) -> &T {
//         self.tag.borrow()
//     }
// }

// pub struct Content<'a, I>
// where
//     I: Iterator,
// {
//     iter: IntoIter<I::Item>,
//     _t: PhantomData<&'a str>,
// }

// impl<'a, I> Content<'a, I>
// where
//     I: Iterator<Item = Event<'a>>
// {
//     pub fn new(iter: IntoIter<I::Item>) -> Content<'a, I> {
//         Content {
//             iter: Some(iter),
//             _t: PhantomData,
//         }
//     }
// }

// impl<'a, I> Iterator for Content<'a, I>
// where
//     I: Iterator<Item = Event<'a>>
// {
//     type Item = Node<'a, Self, Tag<'a>>;

//     fn next(&mut self) -> Option<Node<'a, Self, Tag<'a>>> {
//         let mut iter = None;
//         swap(&mut self.iter, &mut iter);

//         iter.and_then(|i| {
//             Node::try_from(i).map(|(node, _)| node)
//         })
//     }
// }

// pub struct Node<'a, 'b>
// where
//     'a: 'b,
// {
//     tag: Rc<Tag<'a>>,
//     pub content: Content<'a, 'b>,
// }

// impl<'a, 'b> Node<'a, 'b>
// where
//     'a: 'b,
// {
//     pub fn new<InIter>(mut iter: InIter) -> Option<(Node<'a, 'b>, u8)>
//     where
//         InIter: 'b + Iterator<Item = Event<'a>>,
//     {
//         iter.next().map(|i| {
//             match i {
//                 Event::Start(tag) => {
//                     let tag = Rc::new(tag);
//                     let tag2 = tag.clone();

//                     let pred = move |e: &Event<'a>| {
//                         match *e {
//                             Event::End(ref end_tag) => discriminant(tag2.borrow()) == discriminant(end_tag),
//                             _ => true,
//                         }
//                     };
//                     let (content, _rest) = split_by(iter, pred);
//                     let content = Content::new(Box::new(content));
//                     (Node {
//                         tag,
//                         content,
//                     },
//                     0)
//                 },
//                 _ => panic!(),
//             }
//         })
//     }

//     pub fn tag(&self) -> &Tag<'a> {
//         self.tag.borrow()
//     }
// }

// pub struct Content<'a, 'b>
//     where
//     'a: 'b,
// {
//     iter: Option<Box<'b + Iterator<Item = Event<'a>>>>,
// }

// impl<'a, 'b> Iterator for Content<'a, 'b>
// where
//     'a: 'b,
// {
//     type Item = Node<'a, 'b>;

//     fn next(&mut self) -> Option<Node<'a, 'b>> {
//         let mut iter = None;
//         swap(&mut self.iter, &mut iter);
//         iter.and_then(|i| {
//             Node::new(i).map(|(node, _)| node)
//         })
//     }
// }

// impl<'a, 'b> Content<'a, 'b>
// where
//     'a: 'b,
// {
//     pub fn new<I>(iter: Box<I>) -> Content<'a, 'b>
//     where
//          I : 'b + Iterator<Item = Event<'a>>
//     {
//         Content {
//             iter: Some(iter)
//         }
//     }
// }
