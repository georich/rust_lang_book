const MAX_POINTS: u32 = 100_000;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    let x = 5;

    let x = x + 1;

    let x = x * 2; // shadowing allows to do a few transforms but keep a value immutable
    // shadowing is useful to change a variables type
    let spaces = "   ";
    let spaces = spaces.len();
    // let mut spaces = "   " and spaces = spaces.len() would result in a error of not being
    // allowed to mutate a variables type

    // numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // boolean type
    let t = true;
    let f: bool = false;

    // character type
    // char's use single quotes
    let c = 'z';
    let z = 'Z';

    // Compound types
    // tuples are a way of grouping values with various types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // can use pattern matching to destructure a tuple value
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
    // can access a tuple element directly using a period
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arrays in rust have a fixed length
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
