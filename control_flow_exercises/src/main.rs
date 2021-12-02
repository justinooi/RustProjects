use std::io;

fn main() {
    println!("Enter your number for temperature - Celsius or Farenheit:");

    let mut temp_type = String::new();
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line");

    let temp_type = temp_type.trim();

    println!("Enter your temperature:");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("{}", temp_type);
    convert_temperature(temp, temp_type.to_string());
}



fn convert_temperature(temp: i32, temp_type: String) {
    match temp_type.as_str() {
        "Farenheit" => {
            println!("Temperature Selected!");
            let converted_value = ((temp - 32) * 5) / 9;
            println!(
                "{} degrees farenheit is {} degrees celsius",
                temp, converted_value
            );
        }
        "Celsius" => {
            println!("Celsius Selected!");
            let converted_value = ((temp * 9) / 5) + 32;
            println!(
                "{} degrees celsius is {} degrees farenheit",
                temp, converted_value
            );
        }
        "a" => {
            println!("ayy lamo");
        }
        _ => {
            println!("Invalid input!");
        }
    }
}
