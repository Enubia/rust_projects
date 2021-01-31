use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // create a random number from the installed "rand" crate
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // "mut" is used to make variables mutable
        // :: indicates an "associated function", in other languages called "static function"
        let mut guess = String::new();

        io::stdin()
            // "&" indicates that guess is a reference
            .read_line(&mut guess)
            .expect("Failed to read line");

        // we shadow the guess variable here so that we don't need to create a new unique variable name
        // since we only parse the input as unsigned integer
        // we also use a match expression so that we don't crash if the input is not a valid number
        // and proceed with the next loop iteration
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
