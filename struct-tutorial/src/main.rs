#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let other_rect = Rectangle {
        width: 80,
        height: 100,
    };

    let square = Rectangle::square(3);

    println!("Area is: {}", rect.area());
    println!("Area is: {}", square.area());
    println!("Rect1 can hold Other Rect? {}", rect.can_hold(&other_rect))
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
