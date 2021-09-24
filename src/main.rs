use crate::List::Cons;
use std::ops::Deref;
use std::rc::Rc;

mod drop;
// use ::{drop};

enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);

    println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Cons(4, List::Nil))));
    let list = Cons(1, Box::new(Cons(2, Box::new(List::Nil))));

    deref();

    name();

    drop::drop_main();

    rc();
}

enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

fn rc() {
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::<i32>::Nil))),
    ));

    println!("A count: {}", Rc::strong_count(&a));

    let b = RcList::Cons(3, Rc::clone(&a));

    println!("B count: {}", Rc::strong_count(&a));

    {
        let c = RcList::Cons(4, Rc::clone(&a));

        println!("C count: {}", Rc::strong_count(&a));
    }

    println!("-C count: {}", Rc::strong_count(&a));

    drop(b);

    println!("-B count: {}", Rc::strong_count(&a));
}

fn name() {
    let str = "Abiria";

    hello(str);

    let b = MyBox::new(String::from("ame"));

    hello(&b);

    hello(&String::from("I love ame"));
}

fn deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // assert_eq!(5, y);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("hello, {}!", name);
}
