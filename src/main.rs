use std::io;

fn main() {
    println!("Temperature Converter!");

    let temperature_unit: char = loop {
        println!("Input C for Celsius and F for Fahrenheit");
        let mut temperature_unit = String::new();
        io::stdin()
            .read_line(&mut temperature_unit)
            .expect("Failed to read line.");

        match temperature_unit.trim().parse() {
            Ok(unit) => break unit,
            Err(_) => continue,
        };
    };

    let temperature = loop {
        println!("Please input temperature to be converted");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");

        let mut temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if temperature_unit == 'C' || temperature_unit == 'c' {
            temperature = (temperature * 9 / 5) + 32;
        } else if temperature_unit == 'F' || temperature_unit == 'f' {
            temperature = (temperature - 32) * 5 / 9;
        }

        break temperature;
    };

    println!("The converted value is: {}", temperature);
}
