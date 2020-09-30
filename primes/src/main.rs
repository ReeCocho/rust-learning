// Program which computes N prime numbers

use std::io;

fn main() 
{
    // Prompt user for input
    print!("Please enter the number of primes to compute: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    // Value to hold number of primes to compute
    let mut n: i32;

    loop 
    {
        // Read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err()
        {
            println!("\nThere was a problem reading your input");
            print!("Please try again: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            continue;
        }

        // Convert input to an integer
        n = input.trim().parse::<i32>().unwrap_or_else(|_error|
        {
            println!("\nSorry, I didn't quite understand you.");
            print!("Please try again: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            return -1;
        });
            
        // Invalid guess
        if n == -1
        {
            continue;
        }
        // Bounds checking for valid guess
        else if n < 1
        {
            print!("\nPlease enter a number greater than 0: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            continue;
        }

        // Break out of loop
        break;
    }

    // Create vector to hold primes
    let mut primes: Vec<u64> = Vec::<u64>::new();

    // Only 1 prime needed
    if n == 1 
    {
        primes.push(1);
    }
    // More than 1 prime needed
    else  
    {
        // Add first two primes
        primes.push(1);
        primes.push(2);

        // Variable indicating which possible prime we are inspecting
        let mut possible_prime : u64 = 3;

        // Loop until we have the total number of primes
        while primes.len() != n  as usize 
        {

            // Flag indicating if the number is a prime
            let mut is_prime: bool = true;

            // Loop over all primes (excluding the first, which is 1)
            let mut iter = primes.iter();
            iter.next();
            for prime in iter 
            {
                // If the current prime squared is greater than our inspected prime, we can safely
                // ignore it (a%b==0 is false if a*a > b is true)
                if prime * prime > possible_prime 
                {
                    break;
                }

                // Check if it evenly divides our possible prime
                if possible_prime % prime == 0 
                {
                    is_prime = false;
                    break;
                }
            }

            // Add the prime to the list
            if is_prime 
            {
                primes.push(possible_prime);
            }

            // Increment possible prime (skipping even numbers)
            possible_prime += 2;
        }
    }

    // Print all primes
    println!("Primes:");
    print!("[");

    // Only 1 prime
    if primes.len() == 1 
    {
        print!("{}", primes[0]);
    }
    // More than 1 prime
    else 
    {
        // Print N-1 primes with comma
        for i in 0..primes.len() - 1 
        {
            print!("{},", primes[i]);
        }
        
        // Print final prime without comma
        print!("{}", primes[primes.len() - 1]);
    }
    print!("]");
}