use std::io::Write;

// Trim white spaces (enter) with trim method
// Then get every character in the string with chars method
// Then reverse the order of every characters (1, 2, 3 -> 3, 2, 1) with rev method
// Then we collect all separated characters into a collection (String)
fn reverse_string(document: &str) -> String {
    document.trim().chars().rev().collect::<String>()
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear terminal screen

    let mut line = String::new(); // Create new mutable String struct

    print!("Enter a sentence: ");
    if std::io::stdout().flush().is_err() { // Flush so print! function (method) can output to console
        println!("Flush Error");
    }

    let err = std::io::stdin().read_line(&mut line); // User input

    if err.is_err() { // Check if any error in input
        println!("Error in input - {}", err.unwrap_err()); // Shows the error
        return // Exit program
    }

    let reversed = reverse_string(&line); // Reverse the input
    
    println!("\nInput is : {}Reversed is: {}", line, reversed); // Result
}