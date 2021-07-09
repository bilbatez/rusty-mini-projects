use std::io;

fn main() {
    println!("Input Fahrenheit!!");
    let fahrenheit = ask_user_input_for_fahrenheit();
    let celsius = (fahrenheit - 32) * 5 / 9;
    println!("Celsius: {}°C", celsius);
}

fn ask_user_input_for_fahrenheit() -> isize {
    loop {
        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line!!");

        match fahrenheit.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please input a number!!")
        }
    }
}
