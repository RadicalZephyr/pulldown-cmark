use std::borrow::Borrow;
use std::cell::RefCell;
use std::iter::{Iterator, Peekable};
use std::mem::swap;
use std::rc::Rc;

pub struct KeepUntil<I, P>
where
    I : Iterator,
{
    iter: Option<Rc<RefCell<Peekable<I>>>>,
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
                        let do_next = iter.peek().map(|item| {
                            let pred: &P = self.predicate.borrow();
                            pred(item)
                        });

                        match do_next {
                            Some(true) => {
                                self.iter = Some(iter_copy);
                                iter.next()
                            },
                            Some(false) | None => None
                        }
                    });
                    result
                }
            },
            None => None
        }
    }
}

pub struct DropUntil<I, P>
where
    I : Iterator,
{
    iter: Rc<RefCell<Peekable<I>>>,
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
                let result = self.iter.try_borrow_mut().ok().and_then(|mut iter| {
                    let mut do_next;
                    loop {
                        do_next = iter.peek().map(|item| {
                            let predicate: &P = predicate.borrow();
                            predicate(item)
                        });
                        match do_next {
                            Some(true) => iter.next(),
                            Some(false) | None => break,
                        };
                    }

                    match do_next {
                        Some(false) => iter.next(),
                        Some(true) | None => None,
                    }
                });
                result
            },
            None => self.iter.try_borrow_mut().ok().and_then(|mut iter| iter.next())
        }
    }
}

pub fn split_by<I, P>(iter: I, predicate: P) -> (KeepUntil<I, P>, DropUntil<I, P>)
where
    I: Iterator,
    P: Fn(&I::Item) -> bool
{
    let iter: Peekable<I> = iter.peekable();
    let iter: Rc<RefCell<Peekable<I>>> = Rc::new(RefCell::new(iter));
    let predicate = Rc::new(predicate);
    (
        KeepUntil{
            iter: Some(Rc::clone(&iter)),
            predicate: Rc::clone(&predicate),
        },
        DropUntil {
            iter,
            predicate: predicate.into()
        }
    )
}
