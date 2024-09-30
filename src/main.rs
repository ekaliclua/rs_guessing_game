// Standard input/output
use std::io;
// Standard compare ordering
use std::cmp::Ordering;
// Random Random number generator
use rand::Rng;

fn main() {
    println!("Guess the number !");

    // Range expression: start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // Infinite loop
    loop {
        println!("Please input your guess: ");

        // Mutable guess to new String = ""
        let mut guess = String::new();


        // User input and real line and write it to &guess and expect.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Cast guess<string> to guess<u32> with a match.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Match compare guess & secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You WIN !");
                // If player win, then exit the infinite loop.
                break;
            }
        }
    }
}