fn main() {
    println!("Hello, world!");
    let mut hello = String::from("Hi there");
    let result = first_word(&hello);
    println!("{result}");
    hello.clear();
    let a = [1, 2, 3, 4, 5];
    let b = &a[0..2];
    println!("{b:?}")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
