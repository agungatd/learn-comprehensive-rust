fn shared_reference() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}

// fn error_dangling_reference(x: i32) -> &(i32, i32) {
//     let point = (x, 0);
//     return &point;
// }

fn exclusive_reference() {
    // we will change the first tuple element from 1 to 100 and 2 to 200.
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    let y_coord = &mut point.1;
    *x_coord = 100;
    *y_coord = 200;
    println!("point: {point:?}")
}

fn slice() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    
    // uncomment below line to see that a cannot be mutated
    // due to it has been borrowed.
    // a[3] = 3;

    println!("s: {s:?}");
}

fn two_type_string() {
    // there are two type of string in rust.
    // 1. &str is a slice of UTF-8 encoded bytes, similar to &[u8].
    // 2. String is an owned, heap-allocated buffer of UTF-8 bytes.
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}

fn main() {
    // shared_reference();
    // error_dangling_reference(100);
    // exclusive_reference();
    // slice();
    two_type_string()
}