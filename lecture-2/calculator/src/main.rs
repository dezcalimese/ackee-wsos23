use std::io;

fn add(x: u64, y: u64) -> u64 {
    x + y
}

fn subtract(x: u64, y: u64) -> u64 {
    x - y
}

fn multiply(x: u64, y: u64) -> u64 {
    x * y
}

fn divide(x: u64, y: u64) -> u64 {
    x / y
}

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    println!("Welcome to the calculator app!");
    loop {
        println!("Please enter two numbers:");
        io::stdin().read_line(&mut x).expect("Failed to read line");
        let x: u64 = x.trim().parse().expect("Please type a number");

        io::stdin().read_line(&mut y).expect("Failed to read line");
        let y: u64 = y.trim().parse().expect("Please type a number");

        println!("Please select an operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u64 = choice.trim().parse().expect("Please type a number");
        match choice {
            1 => println!("{} + {} = {}", x, y, add(x, y)),
            2 => println!("{} - {} = {}", x, y, subtract(x, y)),
            3 => println!("{} * {} = {}", x, y, multiply(x, y)),
            4 => println!("{} / {} = {}", x, y, divide(x, y)),
            5 => break,
            _ => println!("Please type a number from 1 to 5"),
        }
    }
}
