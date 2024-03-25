struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} year old", person.name, person.age);
}

fn named_struct() {
    let mut peter = Person { name: String::from("Peter"), age: 27 };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person { name:String::from("Jackie"), ..avery};
    describe(&jackie);
}
// 
// struct Point(u16, u16);
// single-field wrappers (called newtypes)
// struct Newtons(f64);
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
enum PlayerMove {
    Pass,                       // simple variant
    Run(Direction),             // tuple variant
    Teleport { x: u32, y: u32 }, // struct variant
}

fn enums() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);
}

fn main() {
    // named_struct();
    enums();

}