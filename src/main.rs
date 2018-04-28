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

    match guess_num {
        1 => {println!("You choose: Caesar Cipher");
              caesar_main();
        },
        2 => println!("You choose: Affine Cipher"),
        3 => println!("You choose: Vinegere Cipher"),
        _  => println!("{} is not a valid option", guess_num),
    };
}

fn caesar_main () {
    println!("Do you want to Encryt or Decrypt your word?");
    println!("Press 1 for Encrypt or 2 for Decrypt");
    let mut encr = String::new();;
        io::stdin().read_line(&mut encr)
        .expect("Failed to read line");
    let guess_encr: u32 = encr.trim().parse().expect("Not a number!");

    match guess_encr {
        1 => {println!("You want to Encrypt");
             caesar_encrypt();
        },
        2 => println!("You want to Decrypt"),
        _  => println!("{} is not a valid option", guess_encr)
    };
}

fn caesar_encrypt () {
    println!("Type your password you want to encrypt");
    let mut password = String::new();;
        io::stdin().read_line(&mut password)
        .expect("Failed to read line");

    println!("Select your key (number between 1 and 26)");
    let mut key_input = String::new();;
        io::stdin().read_line(&mut key_input)
        .expect("Failed to read line");
    let key: u8 = key_input.trim().parse().expect("Not a number!");

    let mut chars_u8: Vec<u8> = password.into_bytes();
    let mut chars_u8 = chars_u8.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    println!("{:?}", chars_u8);

    let mut chars_u8_shifted: Vec<u8> = Vec::new();

    for mut byte in chars_u8 {
        if byte == 32 {
            byte = 32;
    // If statement for small letters
        }
        else if byte >= 97 {
            if (byte + key > 122) {
                byte = byte - 26 + key;
            } else {
                byte += key;
            }
        }
    // If statement for capital letters
         else if byte < 97 {
            if (byte + key > 90) {
                byte = byte - 26 + key;
            } else {
                byte += key;
            }
        }
        chars_u8_shifted.push(byte);
    }
   println!("{:?}", chars_u8_shifted);
   let result = String::from_utf8_lossy(&chars_u8_shifted);
   println!("Result: {}", result);
}
