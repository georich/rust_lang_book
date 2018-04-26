fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // empties the String, equalling ""

    // word still has value 5 here, but there's no more string that we could meaningfully
    // use the value 5 with, word is now totally invalid!

    // STRING SLICES
    let d = String::from("hello world");

    let hello = &s[0..5]; // hello
    let world = &s[6..11]; // world

    // let slice = &s[0..2] == let slice = &s[..2] can drop 0
    //and [3..len] == [3..]
    // [0..len] == [..]

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // has the type &[i32]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // create an iterator over the array of bytes using .iter()
        if item == b' ' { // search for byte representing space using byte literal syntax
            // if we find a space we return the position
            return i;
        }
    }
    // otherwise return length of the string
    s.len()
}

fn first_word(s: &String) -> &str { // string slice is &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
