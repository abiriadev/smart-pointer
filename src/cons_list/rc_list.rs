use std::rc::Rc;

enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

pub fn list_main() {
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
