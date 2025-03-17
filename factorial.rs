// building a recursive factorial calculator

use std::io::{self,Write};

fn factorial(n:u64)->u64{
    if n == 0{
        1
    }else {
        // creating an expression to suit the requirement of the assignment
        let mut temp = n;
        // a for loop for recursive factorial
        for _ in 0..1{
            temp *= factorial(n-1)

        }
        temp
    }
}

fn main(){
    
    let mut digit = String::new();

    print!("Enter a digit:");
    io::stdout().flush().unwrap();

    io::stdin()
       .read_line(&mut digit)
       .expect("Failed to read number");

    
    
    
    let parsed_digit = digit.trim().parse().expect("Invalid number");
    
    println!("{}",factorial(parsed_digit));


}

