fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);
    let v1 = 4;
    println!("{}", v1);
    let v1 = "4";
    println!("{}", v1);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values in the tuple are: {x}, {y}, {z}");
    println!("The values in the tuple are: {}, {}, {}", tup.0, tup.1, tup.2);
    let a = [1, 2, 3, 4, 5];
    let b = [2; 4];
    println!("{}", b);
}
