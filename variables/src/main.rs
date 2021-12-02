fn main() {
    //Mutable Variables
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    //Const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the constant is {}", THREE_HOURS_IN_SECONDS);

    //Shadowing Variables
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    //Usage of shadow variables
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The amount of spaces is {}", spaces);
}
