fn main() {
    // Causing panics
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    // Recoverable errors with Result
    // enum Result<T, E> {
        // Ok(T),
        // Err(E),
    // }
    use std::fs::File;

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };
    // Matching on different errors
    // use std::io::ErrorKind;
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e)
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };

    // Shortcuts for Panic on error: unwrap and expect
    // unwrap will return value inside Ok if it was the Result value
    // unwrap will call panic! macro for us if it was Err
    // let f = File::open("hello.txt").unwrap();

    // expect will let us choose the panic! message
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Propagating Errors
    use std::io;
    use std::io::Read;

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("hello.txt");

    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut s = String::new();

    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }
    // shortcut for propagating errors: ?
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    fn short_read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
}
