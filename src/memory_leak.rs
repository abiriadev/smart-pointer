#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

// enum List<T> {
//     Cons(T, RefCell<Rc<List<T>>>),
//     Nil,
// }

// impl List<T> {
//     fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
//         match self {
//             Cons(_, ref item) => Some(item),
//             Nil => None,
//         }
//     }
// }

use List::{Cons, Nil};

pub fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("first a count: {}", Rc::strong_count(&a));
    println!("next item of a: {:?}", a.tail());

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

    println!("after b count: {}", Rc::strong_count(&a));
    println!("fist count of b: {}", Rc::strong_count(&b));
    println!("next item of b: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("final b count: {}", Rc::strong_count(&b));
    println!("final a count: {}", Rc::strong_count(&a));

    // println!("NEXT: {:?}", a.tail());
}
