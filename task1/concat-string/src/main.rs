// Function that concatenate 2 strings together
fn string_concat(param1: String, param2: String) -> String {
    println!("Param1 = {:?}", param1);
    println!("Param2 = {:?}", param2);

    format!("{} {}", param1, param2)
}

fn main() {
    let text1 = String::from("I Love");
    let text2 = String::from("Rust");

    let output = string_concat(text1, text2);
    println!("Output = {:?}", output);
}
