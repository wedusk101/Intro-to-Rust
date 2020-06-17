use std::io;

fn fact(n: u32) -> u32 {
	if n <= 1 {
	    return 1;
	}
	else {
	    return n * fact(n - 1);
	}
}

fn main() {
    const DEFAULT: u32 = 0;
	let mut input_str = String::new();
	
    println!("Please enter a positive integer.");
	
	io::stdin()
	    .read_line(&mut input_str)
		.expect("Invalid input");
		
	let input_int: u32 = match input_str.trim().parse() {
		Ok(num) => num,
		Err(_) => {
		    println!("Invalid input");
		    DEFAULT
		}
	};
	
	println!("The result is {}", fact(input_int));
}
