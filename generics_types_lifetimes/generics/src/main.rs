#![allow(unused_variables)]
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic type parameter, usually T
// when we use a type parameter name in a function signature we have to declare it before
// goes in angle brackers
// fn largest<T>(list: &[T]) -> T {
// read as, function largest is generic over some type T, has one param list and type of list is a slice of type T
// will return a value of type T

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic data types in struct definitions
struct Point<T> {
    x: T,
    y: T,
} // this struct has generic type T, x and y are BOTH of this type
// generic data types in method definitions
impl<T> Point<T> { // <T> is needed after impl so Rust knows it is a generic type
    fn x(&self) -> &T {
        &self.x
    }
}
// can also methods on concrete types
impl Point<f32> { // method for only f32 types
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointTwo<T, U> { // this struct has different types for T and U that are both generic
    x: T,
    y: U,
}

impl<T, U> PointTwo<T, U> {
    fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}

// generic data types in enum definitions
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
