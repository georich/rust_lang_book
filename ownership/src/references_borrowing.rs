fn main() {
    let s1 = String::from("hello");

    // &s1 creates reference that refers to value of s1 but doesn't own it, so s1 will not be dropped
    let len = calculate_length(&s1); // has a reference to an object as a parameter instead
    // of taking ownership of the value
    // &s1 in call and &String in function

    println!("The length of '{}' is {}", s1, len);

    ///////////
    let s = String::from("hello");
    change(&s);

    //////////
    let mut s2 = String::from("hello");
    change2(&mut s2); // &mut means a mutable reference
    // mutable references have a big restriction, you can only have one mutable reference
    // to a particular piece of data in a particular scope
    // let r1 = &mut s2;
    // let r2 = &mut s2;
    // this will fail
    // can use new scopes to have multiple references, just not simultaneous ones
    //e.g.
    {
        let r1 = &mut s2;
    } // r1 goes out of scope so can make another
    let r2 = &mut s2;

    // also cannot have mutable reference and immutable one
    let r1 = &s;
    let r2 = &mut s;
    // big problem, this would allow r2 to change the immutable reference of r1
}

// having references as function params is referred to as borrowing
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope but because it doesn't have ownership of s1 nothing happens to it

//////////
fn change(some_string: &String) {
    some_string.push_str(", world"); // what happens if we try to modify something we're borrowing?
    // it doesn't work! references like variables are immutable by default
}

fn change2(some_string: &mut String) { // accept a mutable reference
    some_string.push_str(", world");
}
