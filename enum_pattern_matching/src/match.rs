fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1, // Coin::Penny makes up the pattern
            Coin::Nickel => 5, // => separates the pattern and the code to run
            Coin::Dime => 10, // code associated with each arm is an expression, resulting value is value returned
            Coin::Quarter(state) => {
                println!("Nice! A quarter from {:?}!", state);
                25
            },
        }
    }
    // if we call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be
    // Coin::Quarter(UsState::Alaska). the arms only match when we reach Coin::Quarter(state)
    // state will be the value UsState::Alaska. using it in println! gives us the inner state
    // value out of the Coin enum of Quarter variant


    ////// Matching with Option<T>
    // function to take a Option<i32> and if there's a value inside, add one. if not
    // it should return the None value and do nothing
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    ////// the _ placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // () is unit value so nothing will happen
    }
    // however this can be wordy if we only care about one of the cases, can use if let
    let some_u32_value = Some(0u32);
    match some_u32_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // quite wordy for what it does, can rewrite
    if let Some(3) = some_u32_value {
        println!("three");
    } else {
        // something
    }
}
