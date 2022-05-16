fn main() {
    println!("Hello, world!");

    // ** ownership
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x; // makes a copy of 5 and bind it to y. Stored in stack

    let s1 = String::from("hello"); // stored in the heap
    let s2 = s1; // s1 is no longer valid!! s2 takes ownership to ensure memory safety. This is called a 'move', not a shallow copy

    // println!("{}, world!", s1); // won't compile!

    // create a deep copy with .clone()
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward


    // ** references and borrowing
    let len = calculate_length(&s1); // These ampersands represent references, and they allow you to refer to some value without taking ownership of it

    println!("The length of '{}' is {}.", s1, len);

    // references are immutable by default!

    // you can mutate a reference by declaring the variable as mut and passing in a reference as parameter
    change_string(&mut s1);

    let mut s3 = String::from("hello");
    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    println!("{} and {}", r1, r2); // The scopes of the immutable references r1 and r2 end after the println! where they are last used (NLL)
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s3; // no problem
    println!("{}", r3);

    // ** structs

    // use struct update syntax (..) to specify that remaining fields not explicitly set should have the same values as the given instance
    let user1 = build_user(String::from("omar@domain.com"), String::from("onegronm"));
    let user2 = User {
        email: String::from("otto@domain.com"),
        ..user1
    };
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change_string(s: &mut String) {
    s.push_str(", world");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String. Will not compile!

    // Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // This works without any problems. Ownership is moved out, and nothing is deallocated.

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convert string to array of bytes
    
    for (i, &item) in bytes.iter().enumerate() { // create iterator over array of bytes
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

struct User
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

// a tuple struct
struct Color(i32, i32, i32);
