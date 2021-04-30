use std::fmt;
use std::io;

fn main() {
    println!("Enter temperature, with C/F");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");

    let mut nums = String::new();
    let mut chars = String::new(); 

    for letter in input.chars() {
        match letter {
            '0'..='9' | '.' => nums.push(letter),
            'c' | 'C' | 'f' | 'F' => {
                chars.push(letter);
                break;
            },
            ' ' => continue,
            _ => {
                eprintln!("Unexpected symbol: {}", letter);
                return; 
            }
        }
    }

    let temp = nums.parse::<f64>();

    let res = match &chars[..] {
        "c" | "C"   => Temperature::Celsius(temp.unwrap()),
        "f" | "F"   => Temperature::Fahrenheit(temp.unwrap()),
        _           => { 
            eprintln!("No mode selected");
            return;
        }
    };

    println!("{}", res.convert());

}

#[derive(Debug)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    fn convert(&self) -> Temperature {
        match self {
            Temperature::Celsius(c) => Temperature::Fahrenheit(c * 1.8 + 32.0),
            Temperature::Fahrenheit(f) => Temperature::Celsius((f - 32.0) / 1.8),
        }
    }
}

impl fmt::Display for Temperature{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Temperature::Celsius(c)     => write!(f, "{:.4}C", c),
            Temperature::Fahrenheit(fa) => write!(f, "{:.4}F", fa),
        }
    }
}
