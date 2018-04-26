// OWNERSHIP RULES
// 1. each value in rust has a variable that's called its owner
// 2. there can only be one owner at a time
// 3. when the owner goes out of scope, the value will be dropped

fn main() {
    // let s = "hello":
    let mut s = String::from("hello"); // String from string literal
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // print 'hello, world!

    // ways variables and data interact: Move
    let x = 5;
    let y = x;
    // x bound to 5, copy and bind to y. both pushed to stack

    let s1 = String::from("hello");
    let s2 = s1;
    // String data is copied, meaning we copy the pointer and not the data
    // if both were freed from memory it would cause a double free error,
    // therefore when s2 is binded to s1, s1 is made no longer valid
    // println!("{}, world!", s1); will give error of moved value
    // s1 was moved into s2

    // ways variables and data interact: Clone
    // if we do want a deep copy of String we can use the clone method
    let s1 = String::from("hello");
    let s2 = s1.clone(); // may be expensive
    println!("s1 = {}, s2 = {}", s1, s2);

    // STACK-ONLY data: copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // don't call clone, x is still valid and was not moved into y
    // this is because integers are known size and stored on stack, inexpensive to make a copy

    // Ownership and functions
    // passing a variable to a function will move or copy just like assignment
    let s = String::from("hello"); // comes into scope
    takes_ownership(s); // s's value moves into the function and is no longer valid here

    let x = 5; // comes into scope
    makes_copy(x); // x would move but i32 is Copy so it is still good to use afterwards
} // x goes out of scope then s, but since s was moved nothing special happens

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string is dropped

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope
