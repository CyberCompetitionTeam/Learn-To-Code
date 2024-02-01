use std::io;

fn main() {

    let mut equation = String::new();
    let mut again = String::from("y");

    while again == "y" {
        again.clear();
        equation.clear();

        println!("Enter the equation to calculate (e.g. 1 + 2): ");
    
        io::stdin()
            .read_line(&mut equation)
            .expect("Failed to read line");
    
        let equation = equation.trim();
    
        let values: Vec<&str> = equation.split_whitespace().collect();
    
        if values.len() == 3 {
            let a = values[0].parse::<i32>().expect("Not a valid integer");
            let b = values[2].parse::<i32>().expect("Not a valid interger");
            let operator = values[1].parse::<char>().expect("Not a valid operator");
    
            match operator {
                '+' => println!("{a} + {b} = {}", a+b),
                '-' => println!("{a} - {b} = {}", a-b),
                '*' => println!("{a} * {b} = {}", a*b),
                '/' => println!("{a} / {b} = {}", a/b),
                '%' => println!("{a} % {b} = {}", a%b),
                _   => println!("{operator} is not a valid arithmetic operation."),
            }
    
            println!("Would you like to continue? (y/n):");

            io::stdin()
                .read_line(&mut again)
                .expect("Please enter y or n");

            again.pop(); // Pop '\n' off the string. Trim was being funky.

        } else if values.len() > 3 {
            again.push('y');
            println!("Equation too long!");
            println!("Please enter only two numbers with an arithmetic operator between them.");
        } else if values.len() < 3 {
            again.push('y');
            println!("Equation is too short!");
            println!("Please enter only two numbers with an arithmetic operator between them.");
        } 
    }
}
