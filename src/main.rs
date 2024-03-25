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
enum PlayerMove {
    Pass,                       // simple variant
    Run(Direction),             // tuple variant
    Teleport { x: u32, y: u32 }, // struct variant
}

// fn enums() {
//     let m: PlayerMove = PlayerMove::Run(Direction::Left);
//     println!("On this turn: {:?}", m);
// }

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    // the button is pressed
    ButtonPressed(Button),

    // the car is arived at the given floor.
    CarArrived(Floor),

    // the car door's opened
    CarOpened,

    // the car door's closed
    CarClosed
}

type Floor = i32;

#[derive(Debug)]
enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn elevator_events() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}

fn main() {
    // named_struct();
    // enums();
    elevator_events();
}