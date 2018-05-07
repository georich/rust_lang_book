#![allow(unused_variables)]
enum List {
    // Cons(i32, List),
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    // Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use std::rc::Rc;
// use std::cell::RefCell;
use List::{Cons, Nil};

// Defining own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Treating a type like a reference by implementing the Deref trait
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Deref coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Drop trait
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let list = Cons(1,
    //     Box::new(Cons(2,
    //         Box::new(Cons(3,
    //             Box::new(Nil))))));

    // Following the pointer to the value with *
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    // Using Box<T> like a reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    // Dropping a value early with std::mem::drop
    let e = CustomSmartPointer { data: String::from("some data") };
    drop(e);

    // Using Rc<T> to share data
    // let a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
