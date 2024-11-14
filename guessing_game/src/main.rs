use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Please input your guess.");

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("読み込みに失敗しました");
    println!("You guessed: {}", guess);
}
