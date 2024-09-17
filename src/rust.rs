use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("guess the number");

        println!("please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("could not read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Guess and Secret are the same , you win");
                break;
            }
            Ordering::Greater => println!("not the same , greater , incorrect"),
            Ordering::Less => println!("not the same , less , incorrect"),
        };
        println!("{guess}")
    }
}
