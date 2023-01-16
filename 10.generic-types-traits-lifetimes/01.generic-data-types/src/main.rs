struct PointA<T> {
    x: T,
    y: T,
}
impl<T> PointA<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl PointA<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct PointB<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointB<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
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

    println!("*********** Point A **********");

    let point_1 = PointA { x: 1, y: 2 };
    let point_2 = PointA { x: 1.0, y: 2.0 };
    // let wont_work = PointA {x : 1, y: 2.0}; // mismatched types
    println!("point_1.x is {}", point_1.x);
    println!("point_1.y is {}", point_1.y);
    println!("point_2.x is {}", point_2.x);
    println!("point_2.y is {}", point_2.y);
    println!(
        "point_2 distance_from_origin is {}",
        point_2.distance_from_origin()
    );
    println!("point_2.x is {}", point_2.x());

    // Point B
    println!("*********** Point B **********");
    let point_1 = PointB { x: 1, y: 2 };
    let point_2 = PointB { x: 1.0, y: 2.0 };
    let point_3 = PointB { x: 1, y: 2.0 };

    println!("point_1.x is {}", point_1.x);
    println!("point_1.y is {}", point_1.y);
    println!("point_2.x is {}", point_2.x);
    println!("point_2.y is {}", point_2.y);
    println!("point_2.x is {}", point_2.x());
    println!("point_3.y is {}", point_3.y);
    println!("point_3.x is {}", point_3.x());
}

fn find_largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// both can be combined into
fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
