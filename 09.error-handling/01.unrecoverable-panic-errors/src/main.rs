fn main() {
    // rust errors
    // recoverable and unrecoverable
    // For a recoverable error, such as file not found error, we most likely to report the problem
    // to the user and retry the operation
    // unrecoverable errors are always symptoms of bugs, like trying to access a location beyond
    // the end of an array, and so rust stops immediately stops the program.
    // Rust doesn't have exceptions, instead it has the type Result<T, E> for the recoverable
    // errors and for the unrecoverable errors panic! macro that stops the execution when the
    // program encounters  an unrecoverable error.
    //
    println!("Hello, world!");
    panic!("Error message");
    
}
