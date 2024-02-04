// eg15-24
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// use crate::List::{Cons, Nil};
pub use List::{Cons, Nil};
