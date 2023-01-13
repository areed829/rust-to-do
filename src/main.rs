use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let secret_number = 42;

    println!("The secret number is: {}", secret_number);

    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // println!("You guessed: {}", guess);

    // if guess == secret_number {
    //     println!("You win!");
    // } else if guess < secret_number {
    //     println!("Too small!");
    // } else {
    //     println!("Too big!");
    // }
}
