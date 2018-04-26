fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return value to s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into fn which also moves return into s3
} // s3 out of scope and dropped, s2 out but moved so nothing, s1 out of scope and dropped

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string into scope
    some_string // some_string is returned and moves out to calling function
}

// takes and gives back a String
fn takes_and_gives_back(a_string: String) -> String { // a string into scope
    a_string // a string is returned and moves out to calling function
}
