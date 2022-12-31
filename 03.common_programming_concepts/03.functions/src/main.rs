fn main() {
    println!("Hello, world!");
    another_function();
    let result = plus_five(5);
    println!("The value of result is {result}");
}
fn another_function() {
    println!("Hi, from another function");
}
fn plus_five(x: u32) -> u32 {
    let five = 5;
    x + five
}
