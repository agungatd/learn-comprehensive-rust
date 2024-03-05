fn main() {
    run_variables();
}

fn run_variables() {
    // variable binding
    let ten: i32 = 10;
    println!("variable ten= {ten}");

    // signed vs unsigned integers
    // signed has negative value useful for general-calculations (bank balances, temprature, etc.)
    // unsigned for integers that never be negative (counter, memory addresses, array indices).
    let temp_k: i128 = -273;
    let children: u8 = 2;
    println!("Temprature={temp_k} K, and number of children={children}");

    // floating numbers
    let phi: f32 = 3.14;
    println!("phi is: {phi}");

    // characters
    let grade: char = 'A';
    println!("My grade is {grade}")

    // Booleans
    let is_male: bool = true;
    println!("My sex is {is_male}");
}
