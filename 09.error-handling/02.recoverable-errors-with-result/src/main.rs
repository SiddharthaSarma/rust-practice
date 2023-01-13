enum Result<T, E> {
    Ok(T),
    Err(E)
}
use std::fs::File;

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
}

