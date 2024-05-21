use rand::Rng;
use std::cmp::Ordering;
use std::io; // import `io` (the input/ouput library) from `std` (the standard library)

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create a mutable variable bound to a new, empty instance of a `String` to store user input
        let mut guess = String::new(); // `::` indicates that `new` is an associated function (a function that is implemented on a type) of the `String` type

        // Get user guess
        io::stdin()
            // Get input from user
            /* Returns `Result` of type `enumeration` | `Result` has variants `Ok` and `Err` */
            .read_line(&mut guess) // `&` indicates that the argument is a reference 
            
            // Error handling
            /* Returns error message if `Result` is `Err` */
            /* Returns value that `Ok` holds if `Result` is `Ok` */
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print user guess
        println!("You guessed: {guess}"); // `{}` is a placeholder for a value

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}