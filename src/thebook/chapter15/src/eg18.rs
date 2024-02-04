// eg15-18
use std::rc::Rc;

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// pub use crate::List::{Cons, Nil};
// ok:
// pub use self::List::{Cons, Nil};
pub use List::{Cons, Nil};
