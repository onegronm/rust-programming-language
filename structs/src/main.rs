#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associate function (doesn't require an instance)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    }

    println!(
        "The area of the rectangle is {} square pixels.", area(rect1)
    )

    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    rect1.area();

    let square = Rectangle::square(3);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}