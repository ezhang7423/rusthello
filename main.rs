use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess my number!");

    let my_num = rand::random::<u8>();

    let mut tries = 0;
    loop {
        let guess = get_guess();
        match guess.cmp(&my_num) {
            Ordering::Less => println!("Guess bigger"),
            Ordering::Equal => {
                println!("You got it! It took you {} tries", tries);
                break;
            }
            Ordering::Greater => println!("Guess smaller"),
        }
        tries += 1;
    }
}

fn get_guess() -> u8 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("You entered {}", guess);

    return guess.trim().parse().expect("Please type a number!");
}
