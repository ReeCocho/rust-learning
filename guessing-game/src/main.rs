use std::io;
use rand::Rng;

fn main() 
{
    // Main game loop
    loop
    {
        // Generate random number to guess
        let value = rand::thread_rng().gen_range(1, 101);
        
        // Prompt user to begin guessing
        print!("I'm thinking of a number between 1 and 100... Guess what it is!");
        io::Write::flush(&mut io::stdout()).expect("Flush failed!");

        // User guess loop
        loop
        {
            // Read input
            let mut input = String::new();
            match io::stdin().read_line(&mut input)
            {
                Ok(_o) => {}
                Err(_e) => 
                {
                    println!("\nSorry, I didn't quite understand you.");
                    print!("Please try again: ");
                    io::Write::flush(&mut io::stdout()).expect("flush failed!");
                }
            }

            // Convert input to integer
            let mut input_as_int : i32;
            match input.trim().parse::<i32>() 
            {
                Ok(o) =>
                {
                    if o < 1 || o > 100
                    {
                        println!("\nChoose a number between 1 and 100.");
                        print!("Please try again: ");
                        io::Write::flush(&mut io::stdout()).expect("flush failed!");
                    }
                }
            }
        }
    }
}