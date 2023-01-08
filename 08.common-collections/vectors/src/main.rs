fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    let altv = vec![1, 2, 3, 4];

    // reading element
    let third = &v[2];
    println!("The third value in the vector v is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("the third element in vector v is {third}"),
        None => println!("There is no third element"),
    }

    let v = vec![1, 2, 3, 4, 5];

    // panicking
    let does_not_exist = &v[100];

    // won't panic
    let does_not_exist = v.get(100); // returns None if it doesn't find

    /* let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6); // will panic
    println!("The first element is : {first}"); */

    // Iterating over vector
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{i}");
    }

    // iterate over mutable reference
    for i in &mut v {
        *i += 50;
    }

    // storing an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(0.32),
    ];

    {
        let v = vec![1, 2, 3, 4, 5];
    } // <- v goes out of scope and is freed here
}
