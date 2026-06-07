fn main() {
    // let mut s = String::from("hello");
    // s = String::from("ahoy");

    // println!("{s}, world!");

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("hello");  // s comes into scope
    let s2 = s.clone();
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.
    println!("{x}");
    // println!("{s}"); // results in error
    // let s2 = s.clone(); // results in error, because s is moved and borrowed by takes_ownership()
    println!("{s2}");

    main2();

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn main2() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}