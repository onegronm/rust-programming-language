use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn")

    let v = vec![1, 2, 3];

    v[99];

    // RUST_BACKTRACE=1 cargo run

    // ** recoverable errors with Result<T, E>
    // T is type of value returned
    // E is type of error returned

    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap(); // If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.

    // Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier
    // We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// A function that returns errors to the calling code using match
fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // A Shortcut for Propagating Errors: the ? Operator
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // We could even shorten this code further by chaining method calls immediately after the ?
    // let mut s = String::new();

    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // Ok(s)

    // EVEN SHORTER way
    fs::read_to_string("hello.txt")
}