use std::io;

fn main() {
    println!("Welcome to Cipher Machine!");

    println!("You can choose beetwen 3 types of ciphers:");
    println!("- Caesar Cipher (1)");
    println!("- Affine Cipher (2)");
    println!("- Vinegere Cipher (3) ");
    println!("Which cipher do you want to use? Press 1, 2 or 3");

    let mut guess = String::new();;
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess_num: u32 = guess.trim().parse().expect("Not a number!");

    println!("Do you want to encrypt or decrypt your word?");
    println!("Press 1 for Encrypt or 2 for Decrypt");
    let mut encr = String::new();;
        io::stdin().read_line(&mut encr)
        .expect("Failed to read line");
    let guess_encr: u32 = encr.trim().parse().expect("Not a number!");
    
    match guess_num {
        1 => {println!("You chose: Caesar Cipher");
		match guess_encr {
			1 => println!("You want to Encrypt"),
			2 => println!("You want to Decrypt"),
			_  => println!("{} is not a valid option", guess_encr)
		};
	     },
        2 => {println!("You chose: Affine Cipher");
		match guess_encr {
			1 => println!("You want to Encrypt"),
			2 => println!("You want to Decrypt"),
			_  => println!("{} is not a valid option", guess_encr)
		};
	     },
        3 => {println!("You chose: Vinegere Cipher");
		match guess_encr {
			1 => println!("You want to Encrypt"),
			2 => println!("You want to Decrypt"),
			_  => println!("{} is not a valid option", guess_encr)
		};
	     },
         _  => println!("{} is not a valid option", guess_num),
    };
}