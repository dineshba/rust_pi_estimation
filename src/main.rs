use std::io;

fn main() {
    let mut guess = String::new();
    println!("Input your guess: ");

    io::stdin().read_line(&mut guess)
        .expect("Failed to read the input");

    println!("You guessed {}", guess);    
}
