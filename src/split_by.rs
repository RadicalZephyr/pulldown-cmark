use std::cell::RefCell;
use std::fmt::Debug;
use std::iter::{FromIterator, Iterator};
use std::rc::Rc;
use std::vec::IntoIter;

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct TakeWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> TakeWhile<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn collect_with_rest(mut self) -> (Vec<I::Item>, I)
    {
        let mut v = vec![];
        loop {
            if let Some(x) = self.next() {
                v.push(x);
            } else {
                break;
            }
        }
        (v, self.iter)
    }
}

impl<I: Iterator, P> Iterator for TakeWhile<I, P>
    where P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if (self.predicate)(&x) {
                    Some(x)
                } else {
                    self.flag = true;
                    None
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

fn take_while<I, P>(iter: I, predicate: P) -> TakeWhile<I, P> {
    TakeWhile {
        iter,
        flag: false,
        predicate
    }
}

pub fn split_by<I, P>(iter: I, predicate: P) -> (IntoIter<I::Item>, I)
where
    I: Debug + Iterator,
    P: Fn(&I::Item) -> bool
{
    let (keeps, iter): (Vec<_>, I) = take_while(iter, predicate).collect_with_rest();

    (keeps.into_iter(),
     iter)
}
