fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number is: {largest}");

    let number_list = vec![102, 34, 6800, 89, 54, 2, 42, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number is: {largest}");
    let numbers_list = vec![123, 456, 698, 145];
    let largest = find_largest(&numbers_list);

    println!("Largest number is: {largest}");
}

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
