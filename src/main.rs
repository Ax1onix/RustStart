use std::io;
use std::{time::Duration};
use async_std::task;

async fn FaN() {
    println!("You. are. TERRIBLE!");
    task::sleep(Duration::from_millis(5000)).await;
    println!("*Get Force-a-Natured*");
    task::sleep(Duration::from_millis(1000)).await;
}

#[async_std::main]
async fn main() {
    let count: u32 = 1;
    let skull = '\u{1F480}';
    println!("Hello, world!");
    println!(
        "This project is being made only by {} person {}",
        count, skull
    );
    println!("How many people are working on the project?????");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR IN INPUT");

    // Handle input parsing gracefully
    let input: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return; // Exit the program if input is invalid
        }
    };

    if input == 1 {
        loop { // Example: repeat 5 times
            println!("GOOD JOB!!!");
        }
    } else {
        FaN().await;
        loop { // Example: repeat 5 times
            println!("{}{}WRONG!!!!{}{}", skull, skull, skull, skull);
        }
    }
}
