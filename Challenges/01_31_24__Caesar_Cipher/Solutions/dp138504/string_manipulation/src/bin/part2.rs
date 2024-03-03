use std::io;

fn shift_letters(letter: char, shift_amount: u8, cipher_text: &mut Vec<u8>) {
    let mut min_value = 0;

    if letter.is_ascii_lowercase() {
        min_value = 97;
    } else if letter.is_ascii_uppercase() {
        min_value = 65;
    }
        match u8::try_from(letter) {
            Ok(l) => {
                if l - shift_amount < min_value {
                    cipher_text.push(l+26-shift_amount);
                } else {
                    cipher_text.push(l-shift_amount);
                }
            },
            Err(_) => panic!("Eek!"),
        }
}

fn main() {
    let mut user_input = String::new();
    let mut shift_amount = String::new();
    let mut cipher_text: Vec<u8> = Vec::new();
    
    println!("Enter a string to shift");
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Should be a valid string");
    
    println!("Enter a number of characters to shift, e.g. 13");
    
    io::stdin()
        .read_line(&mut shift_amount)
        .expect("Should be a valid integer 0-26");
        
    let shift_amount = shift_amount.trim().parse::<u8>().unwrap();
        
    for letter in user_input.chars() {
        if letter.is_whitespace() {
            cipher_text.push(32); // ASCII space
        } else if letter.is_ascii_alphabetic() { 
            shift_letters(letter, shift_amount, &mut cipher_text);
        } else {
            cipher_text.push(u8::try_from(letter).unwrap());  
        }
    }

    let cipher_text = String::from_utf8(cipher_text).unwrap();
    println!("{cipher_text}");
}
