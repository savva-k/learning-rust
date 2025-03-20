#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 40,
    };

    println!("Rect: {rect:?}");
    println!("Rect area: {}", rect.area());
}
