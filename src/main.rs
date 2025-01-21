use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");
    println!("You can perform addition, subtraction, multiplication, and division.");
    println!("------------------------------------------");

    // Step 1: Get the first number from the user
    let num1 = get_number("Enter the first number:");

    // Step 2: Get the second number from the user
    let num2 = get_number("Enter the second number:");

    // Step 3: Get the operator (+, -, *, /)
    println!("Enter the operation you want to perform (+, -, *, /):");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    // Trim whitespace and get the first character of the operation
    let operation = operation.trim();

    // Step 4: Perform the calculation
    let result = calculate(num1, num2, operation);

    // Step 5: Display the result
    match result {
        Ok(value) => println!("The result of {} {} {} is: {}", num1, operation, num2, value),
        Err(error) => println!("Error: {}", error),
    }
}

/// Function to get a number input from the user
fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

/// Function to perform the calculation
fn calculate(num1: f64, num2: f64, operation: &str) -> Result<f64, String> {
    match operation {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err(String::from("Division by zero is not allowed"))
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(String::from("Invalid operation. Please use +, -, *, or /."))
    }
}
