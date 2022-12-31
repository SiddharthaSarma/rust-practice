fn main() {
    // Rust is stastically typed language.
    let guess = "42".parse().expect("Not a number!");
    // solution
    // let guess: u32 = "42".parse().expect("Not a number!");


    /*
        Rust types
        1. Scalar types (Integer, floating-point, Booleans, characters)
        2. Compund types
    */
    /*
        Integer types
        --------------
        -------------------------
        Length	Signed	Unsigned
        -------------------------
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize
        --------------------------
    -> Signed numbers are stored using two's compliment
    -> Each signed variant can store numbers from -(2 ^ n - 1) to 2 ^ n - 1 - 1 inclusive, where n is the number of bits that variant uses.
    -> Ex: i8 : -(2 ^ 7) to 2 ^ 7 - 1= -128 to 127
    -> Ex: u8 : 0 to 2 ^ 8- 1 = 0 to 255
    -> isize and usize types depend on the architecture( 32 or 64 bit) of the computer your program is running on

        Number literals	 Example
        ---------------------
        Decimal	        98_222
        Hex	            0xff
        Octal	        0o77
        Binary	        0b1111_0000
        Byte (u8 only)	b'A'
    */

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
    
    // Booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // compound types
    let tup : (i32, f64, u8) = (500,2.0, 1);
    // destructuring
    let (x, y, 2) = tup;
    println!("The value of x is {x}");
    let five_hundred = tup.0;
    let two_point_zero = tup.1;
    let one = tup.2;

    // arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    
}

/*
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}


*/
