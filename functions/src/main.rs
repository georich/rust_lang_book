fn main() {
    another_function(5, 6);

    // statements and expressions
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // expressions do not end in semicolons, adding one would make it a statement
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5 // most functions return last expression implicititly (~Ruby), can return early still
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
