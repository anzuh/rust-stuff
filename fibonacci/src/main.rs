use std::io;

fn main() {
    let mut input = String::new();

    println!("Which fibonacci number do you want?");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not parse number");
            return;
        },
    };

    let fib = fibonacci(input);

    println!("The {}th number of the fibonacci sequence is: {}", input, fib);
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
