use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 5);

    // accessing
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);

    // for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hashmap and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // cannot access field_name and field_value
    // println!("{field_name}, {field_value}");
    
    println!("{:?}", scores);


    // overwriting values
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(30);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}
