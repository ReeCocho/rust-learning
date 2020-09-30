use std::io;
use rand::Rng;

fn main() 
{
    // Generate random number to guess
    let value = rand::thread_rng().gen_range(1, 101);
        
    // Prompt user to begin guessing
    println!("I'm thinking of a number between 1 and 100...");
    print!("Guess what it is: ");
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");

    // Main game loop
    loop
    {
        // User guess loop
        let mut user_guess: i32;
        loop
        {
            // Read input
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err()
            {
                println!("\nSorry, I didn't quite understand you.");
                print!("Please try again: ");
                io::Write::flush(&mut io::stdout()).expect("flush failed!");
                continue;
            }

            // Convert input to an integer
            user_guess = input.trim().parse::<i32>().unwrap_or_else(|error|
            {
                println!("\nSorry, I didn't quite understand you.");
                print!("Please try again: ");
                io::Write::flush(&mut io::stdout()).expect("flush failed!");
                return 0;
            });
            
            // Invalid guess
            if user_guess == 0
            {
                continue;
            }
            // Bounds checking for valid guess
            else if user_guess < 1 || user_guess > 100
            {
                println!("\nChoose a number between 1 and 100.");
                print!("Please try again: ");
                io::Write::flush(&mut io::stdout()).expect("flush failed!");
                continue;
            }

            // Break out of loop
            break;
        }

        // Too high
        if user_guess > value
        {
            println!("\nToo high!");
            print!("Try again: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            continue;
        }
        // Too low
        else if user_guess < value
        {
            println!("\nToo low!");
            print!("Try again: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            continue;
        }
        // Correct answer
        else
        {
            break;
        }
    }

    // Win message
    println!("\nYou guessed right! The number I was thinking of was {}!", value);
}