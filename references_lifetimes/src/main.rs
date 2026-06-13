// fn main() {
//     // let r;

//     // {
//     //     let x = 5;
//     //     r = &x;
//     // }

//     // println!("r: {r}");

//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {r}");   //   |       |
//                           // --+       |
// }                         // ----------+

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

// won't compile because lifetime annotations of parameters and return value are missing
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// compiles
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

