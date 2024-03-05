fn main() {
    // run_variables();
    run_arithmetic()
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
    println!("My grade is {grade}");

    // Booleans
    let is_male: bool = true;
    println!("My sex is Male == {is_male}");
}

fn circle_area(radius: f32) -> f32 {
    let phi: f32 = 3.14;
    return phi * radius * radius;
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn run_arithmetic() {
    // * example of function that calculate area of a circle
    // let radius: f32 = 7.001;
    // println!("The area of a circle with radius {radius} is {}", circle_area(radius));

    // test overflow, change i32 to i16
    println!("interproduct result: {}", interproduct(120, 100, 248));
}
