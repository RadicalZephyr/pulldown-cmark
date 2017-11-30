use std::fmt::Debug;
use std::iter::{Iterator, Peekable};
use std::vec::IntoIter;

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct TakeWhile<I, P>
where
    I: Iterator,
    I::Item: Clone,
 {
    iter: Peekable<I>,
    flag: bool,
    predicate: P,
}

impl<I, P> TakeWhile<I, P>
where
    I: Iterator,
    I::Item: Clone,
    P: FnMut(&I::Item) -> bool,
{
    fn collect_with_rest(mut self) -> (Vec<I::Item>, Peekable<I>)
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

impl<I, P> Iterator for TakeWhile<I, P>
where
    I: Iterator,
    I::Item: Clone,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            let do_it = match self.iter.peek() {
                Some(x) => {
                    if (self.predicate)(x) {
                        Some(true)
                    } else {
                        self.flag = true;
                        Some(false)
                    }
                },
                None => None,
            };

            match do_it {
                Some(true) => {
                    self.iter.next()
                },
                Some(false) | None => None,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

fn take_while<I, P>(iter: I, predicate: P) -> TakeWhile<I, P>
where
    I: Iterator,
    I::Item: Clone,
{
    TakeWhile {
        iter: iter.peekable(),
        flag: false,
        predicate
    }
}

pub fn split_by<I, P>(iter: I, predicate: P) -> (IntoIter<I::Item>, Peekable<I>)
where
    I: Iterator,
    I::Item: Clone,
    P: Fn(&I::Item) -> bool
{
    let (keeps, iter): (Vec<_>, Peekable<I>) = take_while(iter, predicate).collect_with_rest();

    (keeps.into_iter(),
     iter)
}
