use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	let secret_guess = rand::thread_rng().gen_range(1, 11);
	// println!("The secret number is {}", secret_guess);
	
    println!("Guess the number!");
	
	loop {
		println!("Please input your guess.");

		let mut guess_str = String::new();

		io::stdin()
			.read_line(&mut guess_str)
			.expect("Failed to read line");

		println!("You guessed: {}", guess_str);

		let guess_int: u32 = match guess_str.trim().parse() {
		    Ok(num) => num,
			Err(_) => {
			    println!("Please type a number!");
			    continue;
			}
		};			

		match guess_int.cmp(&secret_guess) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
			    println!("You win!");
				break;
			}
		}
	}
}