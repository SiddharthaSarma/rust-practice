fn main() {
  // variables and mutability
  let x = 5; // won't work change to let mut x = 5;
  println!("The value of x is {x}");
  x = 6;
  println!("The value of x is {x}");
  // constants
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("3 hours in seconds is: {THREE_HOURS_IN_SECONDS}");
  // shadowing 
  let i = 5;
  let i = i + 1;
  {
    let i = i * 2;
    println!("The value of i in inner scope is: {i}");
  }
  println!("The value of i in outer scope is {i}");

  let spaces = "   ";
  spaces = spaces.len(); // won't work, use shadowing like let spaces = spaces.len()
  println!("spaces is {spaces}");
}
