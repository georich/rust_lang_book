#![allow(unused_variables)]
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
    fn foo(x: i32) {
        // x part is a pattern
        // code here
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // Refutability: Whether a Pattern Might Fail to Match
    // example of an irrefutable pattern would be let x = 5;
    // it will match for any possible value and cannot fail to match
    // example of a refutable pattern is if let Some(x) = a_value
    // if a_value is None rather than Some the Some(x) pattern would not match
    // let Some(x) = some_option_value; // error
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }

    // All the Pattern Syntax
    // Matching Literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // in match scope so y is shadowed to 5 as in Some(5) for x
        _ => println!("Default case, x = {:?}", x),
    }
    // match scope ends so y returns to being 10
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // match multiple patterns with |
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ...
    let x = 5;

    match x {
        1...5 => println!("one through 5"), // inclusive range ...
        _ => println!("something else"),
    }
    // ... is only for numbers or chars
    let x = 'c';

    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Desctructuring to Break Apart Values
    // Desctructuring Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // let Point { x: a, y: b } = p; // variable does not have to match field name
    let Point { x, y } = p;
    // assert_eq!(a, 0);
    assert_eq!(x, 0);
    assert_eq!(y, 7);
    // assert_eq!(b, 7);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructuring Enums
    // pattern to destructure an enum should correspond to how the data stored within
    // the enum is defined
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
    }

    // Destructuring References
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3},
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring Values in a Pattern
    // ignoring an entire value with _
    fn foo_two(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo_two(3, 4);

    // ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        },
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        },
    }

    // ignoring an unused variable by starting its name with an underscore
    let _x = 5;
    let y = 10;

    // ignoring remaining parts of a value with ..
    struct PointTwo {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = PointTwo { x: 0, y: 0, z: 0 };

    match origin {
        PointTwo { x, ..} => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
    // to be unambiguous .. can only be used once per expression

    // ref and ref mut to Create References in Patterns
    // let robot_name = Some(String::from("Bors"));
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        // Some(name) => println!("Found a name: {}", name), // valued moved here, can't be printed/called later
        // Some(ref name) => println!("Found a name: {}", name), // now only referenced
        Some(ref mut name) => *name = String::from("Another name"), // robot name is: Another name
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    // Extra Conditionals with Match Guards
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ Bindings
    enum MessageTwo {
        Hello { id: i32 },
    }

    let msg = MessageTwo::Hello { id: 5 };

    match msg {
        MessageTwo::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        MessageTwo::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        MessageTwo::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
