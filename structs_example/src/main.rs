#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method implementation
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 50,
        height: 30,
    };

    let rectangle2 = Rectangle {
        width: 5,
        height: 5,
    };

    println!("{:#?}", rectangle1);

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rectangle1)
    );

    println!(
        "The area of the rectangle is {} square pixels via struct implementation of methods",
        rectangle1.area()
    );

    if rectangle1.width() {
        println!(
            "The rectangle has a non-zero width, and it is: {}",
            rectangle1.width
        );
    }

    println!(
        "Can Rect2 fit in Rect1? {}",
        rectangle1.can_hold(&rectangle2)
    );

    let square = Rectangle::square(3);
}

// Non-Tuple implementation
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
