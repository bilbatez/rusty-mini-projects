use std::io;

fn main() {
    println!("Fibonacci Finder!");
    println!("Created by Albert JT\n");

    println!("Fibonacci of: ");
    let n = loop {
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line!!");
        match value.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please input a number!!")
        }
    };

    let mut val1 = 0;
    let mut val2 = 1;
    for _ in 1..n {
        let temp = val2;
        val2 += val1;
        val1 = temp;
    }
    println!("Result: {}", val2);
}
