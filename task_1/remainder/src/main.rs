fn remainder_int (param1: i64, param2: i64) -> i64 {
    println!("Param1 = {:?}", param1);
    println!("Param2 = {:?}", param2);

    param1 % param2
}

fn main() {
    let number1 = 10;
    let number2 = 3;

    let remainder = remainder_int(number1, number2);
    println!("Output = {:?}\n", remainder);

    // let number1 = 10;
    let number2 = 2;

    let remainder = remainder_int(number1, number2);
    println!("Output = {:?}\n", remainder);

    let number1 = 20;
    let number2 = 7;

    let remainder = remainder_int(number1, number2);
    println!("Output = {:?}\n", remainder);
}
