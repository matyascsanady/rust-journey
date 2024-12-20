use rand::Rng;
use std::cmp::Ordering;
use std::io; // std::io - Input/Output library from the standard library

fn main()
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop
    {
        println!("Please input your guess.");

        // In rust variables are immutable by default!
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // read_line() APPENDS to the given variable!; & is a reference indicator
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>
            {
                println!("You win!");
                break;
            }
        }
    }
}
