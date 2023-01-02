struct Rectangle {
    width: usize,
    height: usize,
}
impl Rectangle {
    fn area(&self) -> usize {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: usize) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
pub fn get_rect() {
    let rect = Rectangle {
        width: 25,
        height: 25,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 15,
    };
    let sqr = Rectangle::square(16);
    println!("square area is {}", sqr.area());
    println!("method syntax {:?}", rect.area());
    println!("rect2 can fit in rect ? {}", rect.can_hold(&rect2));
    println!("The area of rectangle is {}", rect_area(&rect));
}
fn rect_area(rect: &Rectangle) -> usize {
    rect.width * rect.height
}
