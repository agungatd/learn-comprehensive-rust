
fn main() {
    // Array
    // let mut a: [i8; 10] = [3; 10];
    // a[3] = 13;
    // println!("a: {a:#?}")

    // Tuples
    // let mut t: (i8, bool) = (3, false);
    // t.1 = true;
    // println!("t.0: {}", t.0);
    // println!("t.1: {}", t.1)

    // Array iteration
    // The for statement supports iterating over arrays (but not tuples).
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    // Patterns and destructuring
    
    print_struct()
}
struct Obj {
    a: i32,
    b: bool,
}

fn print_struct(foo: Obj) {
    let Obj { a, b } = foo;
    println!("a: {a}, b: {b}");
}