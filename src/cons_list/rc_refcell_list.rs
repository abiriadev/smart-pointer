use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum RcRefCellList<T> {
    Cons(Rc<RefCell<T>>, Rc<RcRefCellList<T>>),
    Nil,
}

use RcRefCellList::{Cons, Nil};

pub(crate) fn list_main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a: {:?}", a);
    println!("a: {:?}", b);
    println!("a: {:?}", c);
}
