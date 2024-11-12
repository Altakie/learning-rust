use std::io;

fn main() {
    let input_temp: f32 = loop {
        println!("Please enter the temperature (numbers only) :");

        let mut input_temp = String::new();
        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");

        let input_temp: f32 = match input_temp.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Input a valid number!");
                continue;
            }
        };

        break input_temp;
    };

    let result_temp = loop {
        println!("Please input your temp type (F or C)");

        let mut temp_type = String::new();
        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read line");

        let result_temp: f32 = match temp_type.trim() {
            "F" => fahrenheit_to_celcius(input_temp),
            "C" => celcius_to_fahrenheit(input_temp),
            _ => {
                println!("Please input either F or C");
                continue;
            }
        };

        break result_temp;
    };

    println!("Result temp is {result_temp}");
}

fn celcius_to_fahrenheit(temp: f32) -> f32 {
    &temp * (9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celcius(temp: f32) -> f32 {
    (&temp - 32.0) * (5.0 / 9.0)
}
