/*

This program calculates the factorial of a given non-negative integer using recursion 
and tracks the intermediate computation steps. 
The project demonstrates Rust's handling of variables, expressions, conditionals,
loops, functions with ownership and references,
and object-oriented techniques using `struct` and `impl`.

*/

// Importing standard input and output libraries
use std::io::{self,Write};

// Struct to represent a factorial calculator
struct FactorialCalculator{
   // Stores intermediate results of factorial computation
    history: Vec<u64>,

}



impl FactorialCalculator{

    // Constructor to initialize a new FactorialCalculator with an empty history
    fn new() -> Self{
        Self {history: Vec::new()}
    }

     // Recursive function to compute the factorial of a given number
    fn compute_factorial(&mut self, n:u64) -> u64{

        if n == 0{
        // Base case: factorial of 0 is 1
            1
                 
        }else {

            let mut temp = n;
            
            // Loop executes only once, effectively calling recursion
            for _ in 0..1 {

                // Recursive call
                temp *= self.compute_factorial(n - 1); 

                // Store intermediate results in history
                self.history.push(temp); 
            }
            // Return the computed factorial value
            temp 
        }


    }

     // Function to display the history of computed factorial steps
     fn show_history(&self) {
        println!("Intermediate values: {:?}", self.history);
    }

}


fn main() {

    // Variable to store user input

    let mut digit = String::new(); 

    // Prompt the user for input
    print!("Enter a digit: ");

    // Ensures the prompt is displayed before input
    io::stdout().flush().unwrap(); 

    // Read input from the user
    io::stdin()
        .read_line(&mut digit)
        .expect("Failed to read number");

    // Parse the input to an unsigned 64-bit integer
    match digit.trim().parse::<u64>() {
        Ok(n) => {

            // Create a new calculator instance
            let mut calculator = FactorialCalculator::new();

            // Compute factorial
            let result = calculator.compute_factorial(n); 
            
            // Display result
            println!("Factorial of {} is: {}", n, result); 

            // Display intermediate computation steps
            calculator.show_history(); 
        }

        // Handle invalid input
        Err(_) => println!("Invalid input! Please enter a valid non-negative number"), 
    }
}
