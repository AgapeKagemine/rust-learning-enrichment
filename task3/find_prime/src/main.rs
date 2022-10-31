// Helper function for finding prime
fn is_prime(number: &u32) -> bool {
    if *number <= 1 { // 1 is not a prime number
        return false;
    }
    for n in 2..*number { // Loop from 2 to the number provided
        if *number % n == 0 { // if the remainder is 0, then that number is not a prime
            return false;
        }
    }
    true // If reach here, the number given is a prime
}

// Iterate every number in the vector with iter method
// Apply helper function for every number in the vector using map method
// Collect all of the separated numbers from the iterators into one collection and return it as boolean vector
fn find_prime_built_in(numbers: &[u32]) -> Vec<bool> {
    return numbers.iter().map(is_prime).collect(); 
}

// Iterate every number using for ... in ...
// Which then after checking the number is prime or not, push (add) the result to a boolean vector (collection)
fn find_prime(numbers: &[u32]) -> Vec<bool> {
    let mut list = Vec::<bool>::new();
    for number in numbers {
        list.push(is_prime(number));
    }
    list
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen terminal

    let mut numbers = Vec::new();

    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]; // Should i use loop?

    // Create the vector data using loop
    for number in 1..=20 {
        numbers.push(number);
    }

    println!("Built In Function Result");

    let list_built_in = find_prime_built_in(&numbers);
    for (index, num) in numbers.iter().enumerate() { // Borrowed iterator? Enumerate the vector so it also provide the iteration
        // How to enumerate two vector? print the enumerated data and use the index to get the numbers in the other vector
        println!("Is {} a prime number? - {:?}", num, list_built_in.get(index).unwrap()); // Panic if empty
    }

    println!("\nNormal Loop Result");

    let list = find_prime(&numbers);
    for (index, num) in numbers.iter().enumerate() { // Borrowed iterator? can reuse
        // print the enumerated data and use the index to get the numbers in the other vector
        println!("Is {} a prime number? - {:?}", num, list.get(index).unwrap()); // Panic if empty
    }
}
