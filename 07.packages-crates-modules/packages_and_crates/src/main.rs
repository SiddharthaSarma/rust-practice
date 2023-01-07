use crate::garden::vegetables::Asparagus;
pub mod garden;
use std::collections::HashMap; // idiomatic way

use std::fmt;
use std::io;


use std::io::Result as IoResult;
use std::fmt::Result as FmtResult;


fn main() {
    /*
    * A Crate is the smallest amount of code that the Rust compiler considers at a time.
    * Crates can contain modules, and the modules may be defined in other files that get compiled
    * with the crate.
    * 
    *
    * Crate -> Binary Crate and Library Crate
    * Library crates doesn't have a main function
    * 
    *
    * Binary Crate -> src/main.rs
    * Library Crate ->  src/lib.rs
    *
    *
    * If a package contain both src/main.rs and src/lib.rs then it has two crates.
    *
    * A package can have multiple binary crates by placing files in the src/bin directory: each
    * file will be a separate binary crate.
    * 
    * */
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
    let mut map = HashMap::new();
    map.insert(1, 2);
    
}

fn function1() -> fmt::Result {

}

fn function2() -> io::Result<()> {

}

fn function1() -> FmtResult {

}

fn function2() -> IoResult<()> {

}
