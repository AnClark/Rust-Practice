use std::io;

fn main() {
    println!("Convert Fahrenheit to Celcius");

    println!("Input a Fahrenheit temperature: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let farenheit: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("** Invalid input"); 
            0.0
        },
    };

    let celcius = fahrenheit_to_celcius(farenheit);

    println!("Result: {}℃", celcius);
}

fn fahrenheit_to_celcius(F: f64) -> f64 {
    (5.0 / 9.0) * (F - 32.0)
}

