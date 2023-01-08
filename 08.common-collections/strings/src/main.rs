fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from(""); //utf-8 based text
    
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");


    let mut s = String::from("lo");
    s.push('l'); // takes character too
    
    // concatenation of strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // note: s1 has moved here and can no longer be used
    println!("s3 is {s3}");

    // inner definition of +
    // fn add(self, s:&str) -> String { // snip}
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("combined s: {s}");

    let s = format!("{s1}-{s2}-{s3}");
    println!("combined s: {s}");
}
