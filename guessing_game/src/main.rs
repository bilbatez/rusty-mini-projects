use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let greeting = r#"
   _____                       _   _
  / ____|                     | | | |
 | |  __ _   _  ___  ___ ___  | |_| |__   ___
 | | |_ | | | |/ _ \/ __/ __| | __| '_ \ / _ \
 | |__| | |_| |  __/\__ \__ \ | |_| | | |  __/
  \_____|\__,_|\___||___/___/  \__|_| |_|\___|
 | \ | |               | |             | |
 |  \| |_   _ _ __ ___ | |__   ___ _ __| |
 | . ` | | | | '_ ` _ \| '_ \ / _ \ '__| |
 | |\  | |_| | | | | | | |_) |  __/ |  |_|
 |_| \_|\__,_|_| |_| |_|_.__/ \___|_|  (_)
    "#;
    println!("{}", greeting);
    println!("Created by Albert JT");
    println!("--------------------\n");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease input a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}
