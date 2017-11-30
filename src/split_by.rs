use std::cell::RefCell;
use std::fmt::Debug;
use std::iter::Iterator;
use std::rc::Rc;
use std::vec::IntoIter;

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct TakeWhile<I, P> {
    iter: Rc<RefCell<I>>,
    flag: bool,
    predicate: P,
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
            match self.iter.try_borrow_mut().ok() {
                Some(mut iter) => {
                    match iter.next() {
                        Some(x) => {
                            if (self.predicate)(&x) {
                                Some(x)
                            } else {
                                self.flag = true;
                                None
                            }
                        },
                        None => None,
                    }
                },
                None => None,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.borrow().size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

fn take_while<I, P>(iter: Rc<RefCell<I>>, predicate: P) -> TakeWhile<I, P> {
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
    let iter = Rc::new(RefCell::new(iter));
    let keeps: Vec<_> = take_while(Rc::clone(&iter), predicate).collect();

    (keeps.into_iter(),
     Rc::try_unwrap(iter).unwrap().into_inner())
}
