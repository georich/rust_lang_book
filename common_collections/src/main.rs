#![allow(unused_variables)]
fn main() {
    // store more than one value in a single data structure next to each other in memory
    let v: Vec<i32> = Vec::new(); // used type annotation because Rust can't infer type from empty Vector

    // vec! macro will create a vector holding values in it, infers type
    let v2 = vec![1, 2, 3, 4, 5]; // because we've given it i32 values vec! can infer it

    //////
    let mut v = Vec::new();
    // add elements via push
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    ////// reading vectors
    let third: &i32 = &v2[2];
    let third: Option<&i32> = v2.get(2);

    // let does_not_exist = &v[100]; // causes panic
    let does_not_exit = v.get(100); // returns None

    // invalid references
    let mut v3 = vec![1, 2, 3, 4, 5];

    // let first = &v3[0]; // won't work immutable borrow

    // v3.push(6); // won't work mutable borrow

    // iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // to change value reference refers to we must use dereference operator *
        // to get to the value in i before we can use +=
    }

    // using an enum to store multiple types
    // vectors can only store values of the same type, fortunately variants of an enum all fall under the same type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /////////// 
    // STRINGS

    let mut s = String::new(); // new() to create an empty string

    let s = "initial contents".to_string(); // can create String from a string literal
    let s = String::from("initial contents"); // can also use from instead

    let mut s = String::from("foo");
    s.push_str("bar"); // appending with push_str

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);// only works since push_str used a reference to s2

    // push method takes a single char and adds it to String
    let mut s = String::from("lo");
    s.push('l');

    // concatenation with + and format!
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // NOTE s1 now has moved here and cannot be used after (intended behaviour)
    // s3 is now "Hello, world!"

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // difficult to track ownership etc with multiple +'s
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! macro works similar to println! but instead of printing to screen it returns a String
    // this also does not take ownership of any of the parameters
    let s = format!("{}-{}-{}", s1, s2, s3);

    // let s1 = String::from("hello");
    // let h = s1[0];
    // INVALID CODE
    
    // internal representation
    // String is a wrapper over Vec<u8>
    let len = String::from("Hola").len(); // len = 4, the Vec storing String "Hola" is four bytes long
    let len = String::from("Здравствуйте").len(); // len = 24
    // each unicode character takes two bytes of storage
    // can use bytes
    // chars
    // grapheme clusters
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // need to be specific in order to take string slices

    // iterating over a string
    for c in "नमस्ते".chars() { // using chars() separates out and returns six values of type char
       println!("{}", c);
    }

    for b in "नमस्ते".bytes() { // returns each raw byte i.e. 224, 164, 168...
        println!("{}", b); // can be made up of more than one byte, especially non latin characters
    }

    ////// Hash Map
    use std::collections::HashMap;
    // HashMaps store their data on the heap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // this HashMap has keys of String's and values of i32's
    scores.insert(String::from("Yellow"), 50);
    // can also construct a HashMap with collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // zip is creating a vector of tuples where Blue is paired with 10 etc
    // collect is turning the vector of tuples into a HashMap
    // <_, _> is used because it's possible to collect into many different data structures
    // for type params we use underscores and Rust can infer the types that it will contain from the data

    // Ownership
    // types that implement Copy trait, like i32, values are copied into HashMap. values like String will
    // be moved into the hash map and it will become the owner
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are now invalid as the hashmap took ownership

    // Accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // gave the key in order to receive the value
    // result willbe Some(&10), it's Some because get returns an Option<&V> in order to be able
    // to return None if needed. 

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    } // able to iterate over

    // Updating a Hash Map
    // if you .insert() the same key and value twice the previous value will be overwritten
    // only insert if the key has no value, using .entry()
    // return value of entry function is an enum called Entry that represents a value that might exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // won't insert because Blue already has a value
    // or_insert returns the value of the Entry if it exists, if not it inserts it and returns the new value

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

} // vectors are dropped like usual here
