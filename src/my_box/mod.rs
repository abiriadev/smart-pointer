use std::ops::Deref;

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

pub fn my_box_main() {
    deref();
    name();
}

fn deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // assert_eq!(5, y);
}

fn name() {
    let str = "Abiria";

    hello(str);

    let b = MyBox::new(String::from("ame"));

    hello(&b);

    hello(&String::from("I love ame"));
}

fn hello(name: &str) {
    println!("hello, {}!", name);
}
