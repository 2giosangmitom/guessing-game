use std::{
    cmp::Ordering,
    io::{self, Write},
    process,
};

/// Enum representing the type of range value (Min or Max).
enum RangeKind {
    Min,
    Max,
}

/// Function to prompt the user for a number (either min or max).
fn ask_for_value(kind: RangeKind) -> i32 {
    loop {
        // Display appropriate prompt based on the range type
        match kind {
            RangeKind::Min => print!("Enter the min value: "),
            RangeKind::Max => print!("Enter the max value: "),
        };

        io::stdout().flush().expect("Failed to flush stdout"); // Flush to ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Try parsing input into an integer
        match input.trim().parse::<i32>() {
            Ok(value) => return value, // Return the parsed value if successful
            Err(_) => eprintln!("Invalid input! Please enter a valid number."), // Print an error message for invalid input
        }
    }
}

fn main() {
    println!("Welcome to the guessing game!");
    println!("Please define a range for generating a random number!");

    // Get the min and max values from the user
    let min_value = ask_for_value(RangeKind::Min);
    let max_value = ask_for_value(RangeKind::Max);

    // Ensure min_value is less than max_value
    if min_value >= max_value {
        eprintln!("Invalid range! Min value must be less than max value.");
        process::exit(1);
    }

    // Generate a random number within the specified range
    let random_number = rand::random_range(min_value..=max_value);

    // Game loop: User keeps guessing until they find the correct number
    loop {
        print!("Enter your guess: ");
        io::stdout().flush().expect("Failed to flush stdout"); // Flush to display the prompt immediately

        // Read user's guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the guess into an integer
        let guess: i32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Invalid input! Please enter a number.");
                continue; // Ask for input again
            }
        };

        // Compare guess with the randomly generated number
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too large! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the correct number!");
                break; // Exit the loop on a correct guess
            }
        }
    }
}
