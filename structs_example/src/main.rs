#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    area: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.area = self.width * self.height;

        self.area
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if other.height * other.width >= self.height * self.width {
            false
        } else {
            true
        }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
            area: 0,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
        area: 0,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 100,
        area: 0,
    };

    let rect3 = Rectangle {
        width: 100,
        height: 1,
        area: 0,
    };

    println!("{:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("{:#?}", rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);

    println!("{:#?}", square);
}