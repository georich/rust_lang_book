fn main() {
    // enum values can be only one of the variants
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    // now can store the IP data
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1");
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // create instances like such, these are of the same type, IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // e.g. fn route(ip_type: IpAddrKind) { }, call with either V4 or V6

    ///// 
    // can more concisely write the IP data by putting it directly into each enum variant
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // each variant can have different types and amounts of associated data i.e.
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));



    enum Message {
        Quit, // no data
        Move { x: i32, y: i32 }, // anonymous struct
        Write(String), // string
        ChangeColor(i32, i32, i32), // three i32 values
    }
    // could do this with four structs, but this way the variants are all grouped under Message type
    // can more easily define a function to accept all of them now
    // can also define methods on enums using impl
    impl Message {
        fn call(&self) {
            // method body
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enum is defined as follows
    enum Option<T> { // <T> means Some can hold one piece of data of any type
        Some(T),
        None,
    }
    // examples of using Option to hold number and string types
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}
