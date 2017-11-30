use std::borrow::Borrow;
use std::cell::RefCell;
use std::iter::Iterator;
use std::mem::swap;
use std::rc::Rc;

pub struct KeepUntil<I, P> {
    iter: Option<Rc<RefCell<I>>>,
    predicate: Rc<P>,
}

impl<I, P> Iterator for KeepUntil<I, P>
where
    P: Fn(&I::Item) -> bool,
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let mut iter_opt = None;
        swap(&mut self.iter, &mut iter_opt);

        match iter_opt {
            Some(iter_ref) => {
                {
                    let iter_copy = Rc::clone(&iter_ref);
                    let result = iter_ref.try_borrow_mut().ok().and_then(|mut iter| {
                        iter.next()
                    }).and_then(|e| {
                        let pred: &P = self.predicate.borrow();
                        if pred(&e) {
                            self.iter = Some(iter_copy);
                            Some(e)
                        } else {
                            None
                        }
                    });
                    result
                }
            },
            None => None
        }
    }
}

pub struct DropUntil<I, P> {
    iter: Rc<RefCell<I>>,
    predicate: Option<Rc<P>>,
}

impl<I, P> Iterator for DropUntil<I, P>
where
    P: Fn(&I::Item) -> bool,
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let mut pred_opt = None;
        swap(&mut self.predicate, &mut pred_opt);

        match pred_opt {
            Some(predicate) => {
                let predicate: &P = predicate.borrow();
                let result = self.iter.try_borrow_mut().ok().and_then(|mut iter| {
                    let mut item = None;
                    loop {
                        println!("Looping!");
                        item = iter.next();
                        match item {
                            Some(ref item) => {
                                if !predicate(item) {
                                    break;
                                }
                            },
                            None => (),
                        }
                    }
                    item
                });
                result
            },
            None => self.iter.try_borrow_mut().ok().and_then(|mut iter| iter.next())
        }
    }
}

pub trait SplitBy {
    fn split_by<P>(self, P) -> (KeepUntil<Self, P>, DropUntil<Self, P>)
        where
        Self: Sized + Iterator,
        P: Fn(&Self::Item) -> bool;
}

impl<T> SplitBy for T
    where
    T: Sized + Iterator,
{
    fn split_by<P>(self, predicate: P) -> (KeepUntil<T, P>, DropUntil<T, P>)
        where
        Self: Sized,
        P: Fn(&T::Item) -> bool
    {
        let iter = Rc::new(RefCell::new(self));
        let predicate = Rc::new(predicate);
        (
            KeepUntil{
                iter: Rc::clone(&iter).into(),
                predicate: Rc::clone(&predicate),
            },
            DropUntil {
                iter,
                predicate: predicate.into()
            }
        )
    }
}
