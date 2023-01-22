use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Secret number is in range from 1 to 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess.");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Can't read your input");

        let guess = guess.trim().parse::<usize>();

        match guess {
            Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small.\n"),
                Ordering::Greater => println!("Too big.\n"),
                Ordering::Equal => {
                    println!("Secret number: {}", secret_number);
                    println!("You win!\n\n\n");
                    println!("Play again? [Y/N]");

                    let mut answer = String::new();

                    std::io::stdin()
                        .read_line(&mut answer)
                        .expect("Can't read your input");

                    if answer.trim().to_ascii_uppercase() != "Y".to_string() {
                        break;
                    } else {
                        println!("\n\n\n\n\n");
                    }
                }
            },
            Err(_) => println!("Please type a number"),
        }
    }
}
