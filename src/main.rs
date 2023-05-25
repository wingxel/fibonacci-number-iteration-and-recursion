use std::io;
use std::io::Write;

fn main() {
    let mut user_input = get_u_int();
    while user_input != 0 {
        println!("Fibonacci number at index {} is {}", user_input, fibonacci_number(user_input));
        user_input = get_u_int();
    }
}

fn get_u_int() -> u32 {
    let mut input = String::new();
    print!("Enter a positive integer: ");
    io::stdout().flush().expect("TODO: panic message");
    match io::stdin().read_line(&mut input) {
        Err(err) => {
            println!("An error occurred reading input {}", err);
        }
        _ => {}
    }

    let user_input: u32 = match input.trim().parse() {
        Err(err) => {
            println!("Error parsing to int {}", err);
            0
        }
        Ok(num) => {
            num
        }
    };
    user_input
}

fn fibonacci_number(index: u32) -> u64 {
    if index == 0 {
        0
    } else if index == 1 {
        1
    } else {
        fibonacci_number(index - 1) + fibonacci_number(index - 2)
    }
}
