use std::io;

fn main() {
    println!("Enter Your Fahrenheit Temperature");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let fahrenheit: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    fahrenheit_to_celsius(fahrenheit);
}

fn fahrenheit_to_celsius(fahrenheit: f64) {
    let celsius = (fahrenheit - 32.0) / 1.8;
    let rounded_celsius = format!("{:.1}", celsius); // Round to 1 decimal place
	
    println!("Temperature in Celsius is {}", rounded_celsius);
}



//formulae //(F - 32) / 1.8 = C
