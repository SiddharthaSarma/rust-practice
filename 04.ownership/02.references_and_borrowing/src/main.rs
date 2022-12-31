fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("Hi there");
    change(&mut s);
    change(&mut s);
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, and {}", r1, r2, r3);
    println!("output, {s}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
/*
At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid
*/
