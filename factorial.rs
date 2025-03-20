// building a recursive factorial calculator

use std::io::{self,Write};

// initializing a struct
struct FactorialCalculator{
    // The history of the calculator as a vector 
    history: Vec<u64>,
}


// All of these basic requirements:
// Variables (mutable and immutable)

// Expressions

// Conditionals

// Loops

// Functions (ownership or reference)

// One of these additional requirements:

// Use a data structure (e.g. Vec, LinkedList, Hashmap).

// Use slicing.

// Use object oriented techniques using both struct and impl.

impl FactorialCalculator{
    fn new() -> Self{
        Self {history: Vec::new()}
    }

    fn compute_factorial(&mut self, n:u64) -> u64{
        if n == 0{
            1
        }else {
            // creating an expression to suit the requirement of the assignment
            let mut temp = n;
            // a for loop for recursive factorial
            for _ in 0..1{
                temp *= self.compute_factorial(n-1);
                self.history.push(temp)
    
            }
            temp
        }


    }

    fn show_history(&self){
        println!("Intermediate value: {:?}",self.history);
    }
}


fn main(){
    
    let mut digit = String::new();

    print!("Enter a digit:");
    io::stdout().flush().unwrap();

    io::stdin()
       .read_line(&mut digit)
       .expect("Failed to read number");
    


    match digit.trim().parse::<u64>(){
        Ok(n)=>{
            let mut calculator = FactorialCalculator::new();
            let result = calculator.compute_factorial(n);

            println!("Factorial of {} is: {}",n,result);

            calculator.show_history();
        }

        Err(_) => println!("Invalid input! Please enter a valid non-negative number"),
    }


}

