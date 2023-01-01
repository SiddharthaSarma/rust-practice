struct Rectangle {
    width: usize,
    height: usize,
}
pub fn get_rect() {
    let rect = Rectangle {
        width: 25,
        height: 25,
    };
    println!("The area of rectangle is {}", rect_area(&rect));
}
fn rect_area(rect: &Rectangle) -> usize {
    rect.width * rect.height
}
