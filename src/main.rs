use std::io;

fn main() {
    println!("Welcome to Cipher Machine!");

    println!("You can choose beetwen 3 types of ciphers:");
    println!("- Caesar Cipher (1)");
    println!("- Affine Cipher (2)");
    println!("- Vigenere Cipher (3) ");
    println!("Which cipher do you want to use? Press 1, 2 or 3");

    let mut guess = String::new();;
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess_num: u32 = guess.trim().parse().expect("Not a number!");

    match guess_num {
        1 => {println!("You choose: Caesar Cipher");
              caesar_main();
        },
        2 => {println!("You choose: Affine Cipher");
            affine_main();
        },
        3 => {println!("You choose: Vigenere Cipher");
            vigenere_main();
        },
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
        2 => {println!("You want to Decrypt");
             caesar_decrypt();
            },
        _  => println!("{} is not a valid option", guess_encr)
    };
}

fn affine_main () {
    println!("Do you want to Encryt or Decrypt your word?");
    println!("Press 1 for Encrypt or 2 for Decrypt");
    let mut encr = String::new();;
        io::stdin().read_line(&mut encr)
        .expect("Failed to read line");
    let guess_encr: u32 = encr.trim().parse().expect("Not a number!");

    match guess_encr {
        1 => {println!("You want to Encrypt");
             affine_encrypt();
            },
        2 => {println!("You want to Decrypt");
             affine_decrypt();
            },
        _  => println!("{} is not a valid option", guess_encr)
    };
}

fn vigenere_main () {
    println!("Do you want to Encryt or Decrypt your word?");
    println!("Press 1 for Encrypt or 2 for Decrypt");
    let mut encr = String::new();;
        io::stdin().read_line(&mut encr)
        .expect("Failed to read line");
    let guess_encr: u32 = encr.trim().parse().expect("Not a number!");

    match guess_encr {
        1 => {println!("You want to Encrypt");
             vigenere_encrypt();
            },
        2 => {println!("You want to Decrypt");
             vigenere_decrypt();
            },
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

    let chars_u8: Vec<u8> = password.into_bytes();
    let chars_u8 = chars_u8.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    println!("{:?}", chars_u8);

    let mut chars_u8_shifted: Vec<u8> = Vec::new();

    for mut byte in chars_u8 {
        if byte == 32 {
            byte = 32;
    // If statement for small letters
        }
        else if byte >= 97 {
            if byte + key > 122 {
                byte = byte - 26 + key;
            } else {
                byte += key;
            }
        }
    // If statement for capital letters
         else if byte < 97 {
            if byte + key > 90 {
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

fn caesar_decrypt () {
    println!("Type your password you want to decrypt");
    let mut password = String::new();;
        io::stdin().read_line(&mut password)
        .expect("Failed to read line");

    println!("Select your key (number between 1 and 26)");
    let mut key_input = String::new();;
        io::stdin().read_line(&mut key_input)
        .expect("Failed to read line");
    let key: u8 = key_input.trim().parse().expect("Not a number!");

    let chars_u8: Vec<u8> = password.into_bytes();
    let chars_u8 = chars_u8.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    println!("{:?}", chars_u8);

    let mut chars_u8_shifted: Vec<u8> = Vec::new();

    for mut byte in chars_u8 {
        if byte == 32 {
            byte = 32;
    // If statement for small letters
        }
        else if byte >= 97 {
            if byte - key < 97 {
                byte += 26 - key;
            } else {
                byte -= key;
            }
        }
    // If statement for capital letters
         else if byte < 97 {
            if byte - key < 65 {
                byte += 26 - key;
            } else {
                byte -= key;
            }
        }
        chars_u8_shifted.push(byte);
    }
   println!("{:?}", chars_u8_shifted);
   let result = String::from_utf8_lossy(&chars_u8_shifted);
   println!("Result: {}", result);
}

fn affine_encrypt () {
    println!("Type your password you want to encrypt");
    let mut password = String::new();;
        io::stdin().read_line(&mut password)
        .expect("Failed to read line");

    println!("Type key A");
    let mut key_a = String::new();;
        io::stdin().read_line(&mut key_a)
        .expect("Failed to read line");
    let key_a: u32 = key_a.trim().parse().expect("Not a number!");

    println!("Type key B");
    let mut key_b = String::new();;
        io::stdin().read_line(&mut key_b)
        .expect("Failed to read line");
    let key_b: u32 = key_b.trim().parse().expect("Not a number!");

    let chars_u8: Vec<u8> = password.into_bytes();
    let chars_u8 = chars_u8.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    let mut chars_u8_shifted: Vec<u8> = Vec::new();

    for mut byte in chars_u8 {
        let mut x = byte as u32;
        println!("{}", x);
        x = ((key_a * x + key_b) % 26) + 65;
        if x >= 78 {
            x -= 13;
        } else {
            x += 13;
        }
            chars_u8_shifted.push(x as u8);
        }

    println!("{:?}", chars_u8_shifted);
    let result = String::from_utf8_lossy(&chars_u8_shifted);
    println!("Result: {:?}", result);
}

fn affine_decrypt () {
    println!("Type your password you want to decrypt");
    let mut password = String::new();;
        io::stdin().read_line(&mut password)
        .expect("Failed to read line");

    println!("Type key A");
    let mut key_a = String::new();;
        io::stdin().read_line(&mut key_a)
        .expect("Failed to read line");
    let key_a: u32 = key_a.trim().parse().expect("Not a number!");

    println!("Type key B");
    let mut key_b = String::new();;
        io::stdin().read_line(&mut key_b)
        .expect("Failed to read line");
    let key_b: u32 = key_b.trim().parse().expect("Not a number!");

    let chars_u8: Vec<u8> = password.into_bytes();
    let chars_u8 = chars_u8.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    let mut chars_u8_shifted: Vec<u8> = Vec::new();

    let mut inv = 0;
    for i in 0..26 {
        let temp = (key_a * i) % 26;
     if temp == 1 {
         inv = i;
         break;
     }
    }

    for mut byte in chars_u8 {
        let mut x = byte as u32;
        println!("{}", x);
        x = (inv * (x - key_b) % 26) + 65;
        if x >= 78 {
            x -= 13;
        } else {
            x += 13;
        }
            chars_u8_shifted.push(x as u8);
        }

    println!("{:?}", chars_u8_shifted);
    let result = String::from_utf8_lossy(&chars_u8_shifted);
    println!("Result: {:?}", result);
}

fn vigenere_encrypt () {
    println!("Type your password you want to encrypt");
    let mut password = String::new();;
        io::stdin().read_line(&mut password)
        .expect("Failed to read line");

    println!("Type key text");
    let mut key_text = String::new();;
        io::stdin().read_line(&mut key_text)
        .expect("Failed to read line");

    let chars_u8: Vec<u8> = password.into_bytes();
    let chars_u8 = chars_u8.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    let chars_key: Vec<u8> = key_text.into_bytes();
    let chars_key = chars_key.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    let mut chars_key_full: Vec<u8> = Vec::new();
    let mut chars_u8_shifted: Vec<u8> = Vec::new();

    println!("{:?}", chars_u8);
    println!("{:?}", chars_key);

    let mut temp = 0;
    while chars_key_full.len() < chars_u8.len() {
        let v = chars_key[temp];
        chars_key_full.push(v);
        temp = temp + 1;
        if temp >= chars_key.len() {
        temp = 0;
        }
    }
    println!("{:?}", chars_key_full);

    temp = 0;
    for mut chars in chars_u8 {
        let mut x: i32 = chars as i32 + chars_key_full[temp] as i32;
        temp = temp + 1;
        if x > 155 {
            chars = (x - 91) as u8;
        } else {
            chars = (x - 65) as u8;
        }
        chars_u8_shifted.push(chars);
    }
    println!("{:?}", chars_u8_shifted);
    let result = String::from_utf8_lossy(&chars_u8_shifted);
    println!("Result: {}", result);
}

fn vigenere_decrypt () {
    println!("Type your password you want to decrypt");
    let mut password = String::new();;
        io::stdin().read_line(&mut password)
        .expect("Failed to read line");

    println!("Type key text");
    let mut key_text = String::new();;
        io::stdin().read_line(&mut key_text)
        .expect("Failed to read line");

    let chars_u8: Vec<u8> = password.into_bytes();
    let chars_u8 = chars_u8.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    let chars_key: Vec<u8> = key_text.into_bytes();
    let chars_key = chars_key.into_iter().filter(|&i|i != 10 && i != 13).collect::<Vec<_>>();
    let mut chars_key_full: Vec<u8> = Vec::new();
    let mut chars_u8_shifted: Vec<u8> = Vec::new();

    println!("{:?}", chars_u8);
    println!("{:?}", chars_key);

    let mut temp = 0;
    while chars_key_full.len() < chars_u8.len() {
        let v = chars_key[temp];
        chars_key_full.push(v);
        temp = temp + 1;
        if temp >= chars_key.len() {
        temp = 0;
        }
    }
    println!("{:?}", chars_key_full);

    temp = 0;
    for mut chars in chars_u8 {
        let mut x: i32 = chars as i32 - chars_key_full[temp] as i32;
        temp = temp + 1;
        if x >= 0 {
            chars = (x + 65) as u8;
        } else {
            chars = (x + 91) as u8;
        }
        chars_u8_shifted.push(chars);
    }
    println!("{:?}", chars_u8_shifted);
    let result = String::from_utf8_lossy(&chars_u8_shifted);
    println!("Result: {}", result);
}
