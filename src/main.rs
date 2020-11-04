use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I've picked a number! You try and guess it 😉");

    loop {
        println!("Please provide your guess 👀:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Wrong! Too small"),
            Ordering::Greater => println!("Wrong! Too big!"),
            Ordering::Equal => {
                println!("🎉");
                println!("You win the interwebz! 🥳");
                println!("🎉");
                break;
            },
        }
    }
}
