use std::borrow::Borrow;
use std::iter::Iterator;
use std::mem::swap;
use std::rc::Rc;

pub struct KeepUntil<I, P> {
    iter: Option<Rc<I>>,
    predicate: Rc<P>,
}

pub struct DropUntil<I, P> {
    iter: Option<Rc<I>>,
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

        iter_opt.as_mut().and_then(|iter_ref| {
            Rc::get_mut(iter_ref)
        }).and_then(|iter| {
            iter.next()
        }).and_then(|e| {
            let pred: &P = self.predicate.borrow();
            if pred(&e) {
                None
            } else {
                Some(e)
            }
        })
    }
}

impl<I, P> Iterator for DropUntil<I, P>
where
    P: Fn(&I::Item) -> bool,
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        None
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
        let iter = Rc::new(self);
        let predicate = Rc::new(predicate);
        (
            KeepUntil{
                iter: iter.clone().into(),
                predicate: predicate.clone()
            },
            DropUntil {
                iter: iter.into(),
                predicate
            }
        )
    }
}
