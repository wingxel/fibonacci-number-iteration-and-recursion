use std::io;
use std::io::Write;

fn main() {
    println!("\n\nPress Ctr+C to exit or enter 0\nUsing the iteration method\n");
    let mut user_input = get_u_int();
    // Test iteration
    // Continue asking user for input until zero is entered
    while user_input > 0 {
        println!(
            "The fibonacci number at index {} = {}", user_input,
            fibonacci_number_loop(user_input)
        );
        user_input = get_u_int();
    }
    println!("\n\nPress Ctr+C to exit or enter 0\nUsing the recursion method\n");
    user_input = get_u_int();
    // Test iteration
    // Continue asking user for input until zero is entered
    while user_input > 0 {
        println!(
            "The fibonacci number at index {} = {}", user_input,
            fibonacci_number_recursion(user_input)
        );
        user_input = get_u_int();
    }
}

// Get the index from the user
fn get_u_int() -> u32 {
    let mut input = String::new();
    print!("Enter a positive integer: ");
    io::stdout().flush().expect("Error flushing stdout");
    match io::stdin().read_line(&mut input) {
        Err(err) => {
            println!("An error occurred reading input {}", err);
        }
        _ => {}
    }
    // Try to convert string to int
    match input.trim().parse() {
        Err(err) => {
            println!("Error parsing to int {}", err);
            0
        }
        Ok(num) => {
            num
        }
    }
}

// Using recursion to get the nth fibonacci number
fn fibonacci_number_recursion(index: u32) -> u64 {
    if index == 0 {
        0
    } else if index == 1 {
        1
    } else {
        fibonacci_number_recursion(index - 1) + fibonacci_number_recursion(index - 2)
    }
}

// Using iteration to get the nth fibonacci number
// Using loop is faster and memory efficient that using recursion
// u64 will overflow on the 94th index
fn fibonacci_number_loop(index: u32) -> u64 {
    // For zero current = -1 and next = 1 so fib_num = 0
    // after that current = 1 and next = 0
    if index == 0 {
        return 0;
    }
    // continue from there using a loop
    let (mut current, mut next, mut fib_num) = (1, 0, 0);
    for _ in 1..=index {
        fib_num = current + next;
        current = next;
        next = fib_num;
    }
    fib_num
}
