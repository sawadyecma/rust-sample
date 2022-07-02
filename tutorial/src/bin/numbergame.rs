use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    println!("Guess the number!");          // 数を当ててごらん
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number); 

    loop{
        println!("Please input your guess.");   // ほら、予想を入力してね

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");    // 行の読み込みに失敗しました

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);   
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       //小さすぎ！
            Ordering::Greater => println!("Too big!"),      //大きすぎ！
            Ordering::Equal => {
                println!("You win!");        //やったね！
                break;
            }
        }
    }

}
