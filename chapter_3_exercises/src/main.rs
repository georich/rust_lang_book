fn main() {
    // Temperature Convert
    let temp_cels = fahrenheit_to_celsius(90);
    println!("90 Fahrenheit in Celsius is: {}", temp_cels);

    let temp_fahrenheit = celsius_to_fahrenheit(25);
    println!("25 Celsius in Fahrenheit is: {}", temp_fahrenheit);

    let temperature: i32 = 25;
    let target: char = 'F';
    let temp = temperature_converter(temperature, target);
    println!("{} in {} is: {}", temperature, target, temp);

    // Fibonacci number generator


    // Print lyrics to Christmas carol "Twelve days of Christmas"
}

fn fahrenheit_to_celsius(temp: i32) -> i32 {
    temp - 32 * 5 / 9
}

fn celsius_to_fahrenheit(temp: i32) -> i32 {
    temp * 9 / 5 + 32
}

fn temperature_converter(temp: i32, target: char) -> i32 {
    if target == 'C' {
        temp - 32 * 5 / 9
    } else if target == 'F' {
        temp * 9 / 5 + 32
    } else {
        0
    }
}
