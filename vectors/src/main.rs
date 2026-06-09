    fn main() {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        let fourth: &i32 = &v[3];
        println!("The fourth element is {fourth}");

        let first: Option<&i32> = v.get(0);
        println!("The first element is {}", first.unwrap());
    
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{}", *i);
        }

        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        let row_first: &SpreadsheetCell = &row[0];
        println!("{row_first:?}");
        let row_second: &SpreadsheetCell = &row[1];
        println!("{row_second:?}");
    }
    
