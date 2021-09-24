#[derive(Debug)]
enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

pub fn list_main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    println!("{:?}", list);
}
