fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measures(5, 'h');
    test();
    println!("The value of x is {}", five());
    println!("The value of 5 + 1 is {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn print_labeled_measures(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn test() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}