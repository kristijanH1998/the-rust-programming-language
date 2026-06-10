fn main() {
    use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // // map.insert(field_name, field_value);
    // map.insert(field_name.clone(), field_value.clone());
    // println!("{field_name}, {field_value}");

    // // scores.insert(String::from("Blue"), 10);
    // // scores.insert(String::from("Blue"), 25);

    // // println!("{scores:?}");

    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    // 1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let nums: [i32; 9] = [3, 3, 2, 5, 1, 4, 6, 6, 6];
    let numslen = nums.len();
    let mut vec: Vec<i32> = Vec::with_capacity(numslen);
    for n in nums {
        vec.push(n);
    }
    println!("{:?}", vec);
    vec.sort();
    println!("{:?}", vec);
    // println!("{numslen}");
    // let mut even: bool = false;
    let mut med: f64 = 0.0; 
    if numslen % 2 == 0 {
        // even = true;
        let mid2 = numslen / 2;     
        let mid1 = mid2 - 1;    
        med = (&vec[mid1] + &vec[mid2]) as f64 / 2.0;
    } else {
        med = vec[numslen / 2] as f64;
    }
    println!("Median of the list is: {}", med);
    
    let mut map_nums: HashMap<i32, i32> = HashMap::new();
    let mut max_count = 0;
    let mut mode: i32 = 0;
    for n in vec {
        // println!("{}", n);
        let count = map_nums.entry(n).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = n;
        }
    }
    println!("{:?}", map_nums);
    println!("Mode of the list is: {}", mode);
    
    // 2. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
}
