// Create a number guessing game
// To obtain user input and print result as output, need to use io (input/output) library
// Alteratively, one may fore the use statement and call th std library using std::io::func()
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Range annoation is start..=end, inclusive.
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Please input guess number.");

    // The loop expression creates an infinite loop until condition reached where breaks
    loop {
        // Declaring a mutable variable with the let statement
        // :: syntac state that new is a method associated with type String
        // This declares a new, empty instance of String as a mutable variable named guess
        let mut guess = String::new();

        // Calls stdin function from io library
        io::stdin()

            // & indicates that argument is a reference.
            // References allow multiple pieces of code to access same piece of data without copying it
            // References are immutable by default, thus why must specify &mut
            .read_line(&mut guess)

            // The read_line function returns a value called result, which is a enum.
            // Enums in rust are types which have multiple different possible state, each being called a variant.
            // The variants of result are Ok or Err, where Err occurs when operation had failed.
            // If the instance of Result is Err, the expect method will cause the program to crash
            // If isntance of Result is Ok, expect will return the value that Ok returns.
            // It is optional to include the expect method, however Rust will warn you that error is no handled.
            // The correct way to suppress this warning is via error handling, but it will be covered in Chapter 9.
            .expect("Failed to read line");

        // Rust allows variable shadowing, allowing us to reuse variables
        // Shadowing is done with the let keyword, otherwise
        // Trim eliminates whitespace. To input a number, user must press enter which introduces /n.
        // Parse converts a string to a different type.
        // Parse returns a result type, if parse does not return a u32 it will return Err
        // upon which it will re-enter the loop until correct type is supplied.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print user number
        println!("You guessed: {guess}");


        // Comparing secret number with user number
        // Ordering from trait cmp is an enum with less, greater and equal variants
        // The match expression executes code based on variant of Ordering.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner winner chicken dinner");
                break;
            }
        }
    }

    println!("The secret number was: {secret_number}")
}
