fn main() {
    // Handling if conditions
    let mut number = 7;

    if number < 5 {
        println!("True condition");
    } else {
        println!("False condition");
    }

    // No automatic conversion of value to boolean in Rust.
    if number != 0 {
        println!("number was something other than zero");
    }

    // Multiple if-else statements
    number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("number is divisible by 2.")
    } else {
        println!("number is not visibile by 4, 3, or 2.")
    }

    //If as expression

    let condition = true;
    number = if condition { 5 } else { 6 };

    println!("The value of the number is: {}", number);

    //Loops
    loop_function();
    return_value_from_loops();
    while_loops();
    for_loops();
}

fn loop_function() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count: {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {}", count);
}

fn return_value_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result)
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFT OFF!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value of element is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFT OFF!");
}
