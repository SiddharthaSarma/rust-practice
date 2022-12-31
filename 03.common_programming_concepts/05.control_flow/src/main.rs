fn main() {
    let number = 3;
    if number < 4 {
        println!("number is less than 4");
    } else {
        println!("number is greater than 4");
    }

    let condition = true;
    let value = if condition { 5 } else { 4 };
    println!("The value is {value}");

    // loops
    // loop, while, for
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is : {}", element);
    }

    for number in (1..10).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
