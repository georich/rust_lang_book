#![allow(unused_variables)]
fn main() {
    // Defining a Trait
    pub trait Summarizable {
        // fn summary(&self) -> String;
        // Default implementation
        fn summary(&self) -> String {
            String::from("(Read more...)")
        }
    }
    // inside braces we declare method signatures that describe the behaviours that types that
    // implement this trait will need to have, in this case fn summary(&self) -> String
    // Each type that implements this trait must then provide custom behaviour for the body of
    // the method. Compiler will enforce that any type with Summarizable trait will have a method
    // summary() defined for it with this signature exactly

    // Implementing a Trait on a Type
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summarizable for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    // Trait Bounds
    pub fn notify<T: Summarizable>(item: T) { // trait bound specifies that item must be of a type
        println!("Breaking news! {}", item.summary()); // that implements Summarizable trait
    }
    // multiple trait bounds
    use std::fmt::{Display, Debug};

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
        // do something
        2
    }
    // alternate syntax
    fn some_function_where<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {
        // do something
        2
    }
}
