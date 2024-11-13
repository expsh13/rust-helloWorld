use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("読み込みに失敗しました");
    println!("You guessed: {}", guess);
}
