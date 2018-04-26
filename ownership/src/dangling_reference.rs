fn main() {
    let reference_to_nothing = no_dangle();
}

fn dangle() -> &String {
    // dangle returns reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a referencce to the String, s
} // here s goes out of scope, and is dropped, it's memory goes away, PROBLEM
  // the reference will point to an invalid String, solution:
fn no_dangle() -> String {
    // SOLUTION
    let s = String::from("hello");

    s
}
