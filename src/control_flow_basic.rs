fn age_cond(age: u32) -> String {
    // the if conditional is similar with other languages
    if age < 18 {
        "You are too young!".to_string()
    } else if age < 60 {
        "work, work, work!".to_string()
    } else {
        "You are a senior!".to_string()
    }
}

fn loops() {
    let mut x = 200;
    // while loop
    while x >= 10 {
        x = x / 2;
    }
    println!("x: {x}");

    // for loop
    for n in 1..=x {
        println!("for n = {n}");
    }

    for elem in [1,2,3] {
        println!("element no: {elem}");
    }

    // `loop` statment just loops forevah until a `break`. use when:
    // - When you don't know in advance how many times the loop should run.
    // - You need a loop that's guaranteed to run at least once.
    // - You can use the break keyword for flexible termination based on various factors within the loop's body.
    loop {
        x += 1;
        if x > 10 {
            break;
        }
    }
    println!("x after loop: {x}");
}

fn loops_break_label() {
    let s = [
        [[5, 6, 7], [8, 9, 10], [21, 15, 32]],
        [[5, 6, 7], [8, 9, 77], [21, 15, 32]],
        [[5, 6, 7], [8, 9, 10], [21, 15, 32]]
    ];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        'inner2nd: for j in 0..=2 {
            println!("inside inner2nd");
            for k in 0..=2 {
                elements_searched += 1;
                if s[i][j][k] == target_value {
                    println!("break");
                    // break; if no label, will go to inner2nd loop
                    break 'outer;
                }
            }
        }
    }
    println!("elements searched: {elements_searched}");
}

fn scope_shadows() {
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    // "a" will be 10, unaffected by the "a" inside the inner scope
    println!("after: {a}"); 
}

fn collatz_length(mut n: u32) -> u32 {
    let mut sequence = 1;

    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        sequence += 1;
    }

    sequence
}

fn main() {
    let age: u32 = 18;
    let x = age_cond(age);
    // if as an expression
    let y = if age < 18 {"Young!"} else {"Adult"};
    println!("the age category: {x}");
    println!("the age is: {y}");
    loops();
    loops_break_label();
    scope_shadows();
    println!("sequence for n=5: {}", collatz_length(5));
}