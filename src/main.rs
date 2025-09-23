use std::io;

fn main() -> io::Result<()> {
    let count: u32 = 1;
    let skull = '\u{1F480}';
    println!("Hello, world!");
    println!(
        "This project is being made only by {} person {}",
        count, skull
    );
    println!("How many people are working on the project?????");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input == "1" {
        loop {
            println!("GOOD JOB!!!");
        }
    } else {
        loop {
            println!("{}{}WRONG!!!!{}{}", skull, skull, skull, skull);
        }
    }
}
