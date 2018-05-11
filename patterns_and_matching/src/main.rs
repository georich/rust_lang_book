fn main() {
    // Condition if let Expressions
    let favorite_color: Option<&str> = None; // Some("blue");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favourite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for Loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let Statements
    let x = 5; // x is just a simple pattern
    let (x, y, z) = (1, 2, 3); // matches (x, y, z) to (1, 2, 3) and matches respective positions
    // let (x, y) = (1, 2, 3); would cause compiler error because of mismatched types

    // Function Parameters
    fn foo(x: i32) { // x part is a pattern
        // code here
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
