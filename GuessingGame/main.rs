use std::io;
/* C-Style comments apparently */
fn main() {
	// four spaces is the Rust style
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

	println!("Please input your guess.");

	let mut guess = String::new(); // variable are immutable by default; ::new() is a static (associated) method of the String type

	io::stdin().read_line(&mut guess) // references (&) are also immutable by default 
	    .expect("Failed to read line"); // readline returns a value of io::Result
	// Result types are enumerations - has value of either Ok or Err
	// inside Ok is the successfully generated value, and inside Err is error info
	// io::Result has an expect method
	// Ok value would be the number of bytes User inputted into standard input
	// would get a warning without expect

	println!("You guessed : {}", guess); // {} = placeholder for "guess"	

	/* A binary crate is an executable. The rand crate is a library crate */
    /* need to modify the Cargo.toml file to include the rand crate as a dependency */

    /* 

[dependencies]

rand = "0.3.14"

    */

    // [section] denotes a section within the Cargo.toml file
    // 0.3.14 is shorthand for ^0.3.14 which means "any version that has a public API compatible with version 0.3.14"

    // Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use

    // Cargo.lock file keeps all dependencies working at their initial versions unless you explicity upgrade

    
    // left off: "First, we add a use line: use rand::Rng"

}
