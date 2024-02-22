use std::io;

fn main() {
    let mut user_input = String::new();
    let mut char_insert = String::new();
    let mut index = String::new();

    println!("Print a string to be reversed");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Should be a valid String");
    
    // Reverse string.
    for character in user_input.chars().rev() {
        print!("{character}");
    }
    println!();
    println!("Enter a character to insert into the string");

    io::stdin()
        .read_line(&mut char_insert)
        .expect("Should be a valid String");

    println!("Enter the index at which to insert the character (0 = 1st character)");

    io::stdin()
        .read_line(&mut index)
        .expect("Should be a valid String");
    
    // Insert character at index and print string.
    {
        let char_insert = char_insert.trim().parse::<char>().unwrap();
        let index = index.trim().parse::<usize>().unwrap();

        user_input.insert(index, char_insert);

        println!("{user_input}");
    }
    
    index.clear();

    println!("Enter the index at which to delete a character (0 = 1st character)");
    
    io::stdin()
        .read_line(&mut index)
        .expect("Should be a valid String");
    
    // Remove character at index
    {
        let index = index.trim().parse::<usize>().unwrap();
        user_input.remove(index);

        println!("{user_input}");
    }
}
