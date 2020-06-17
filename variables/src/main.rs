use std::io;
use std::cmp::Ordering;

fn main() {
	const VAL: u32 = 5;
	const DEFAULT: u32 = 0;
    println!("Please enter an integer.");
	
	let mut input_str = String::new();
	
	io::stdin()
	    .read_line(&mut input_str)
		.expect("Enter a number");
		
	let input_int: u32 = match input_str.trim().parse() {
		Ok(num) => num,
		Err(_) => DEFAULT // returns a default value
	};
	
	match input_int.cmp(&VAL) {
	    Ordering::Less => println!("Less"),
		Ordering::Greater => println!("Greater"),
		Ordering::Equal => println!("Equal")
	};
}
