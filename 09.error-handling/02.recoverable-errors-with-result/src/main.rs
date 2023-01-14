// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
use std::{fs::File, io::ErrorKind, io::Read};
// use std::io::self;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    // other method
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt").expect("Not able to open the file");

    // propagating errors
}

fn read_username_from_file() -> std::result::Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_query_version() -> std::result::Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_other_version() -> std::result::Result<String, std::io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/*
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
*/
