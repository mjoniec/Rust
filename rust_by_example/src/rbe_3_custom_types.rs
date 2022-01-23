#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
//#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

pub fn structures(){
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?} {:?} {:?}", peter, peter.name, peter.age);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { y: 3.3, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let bottom_right2 = Point { x:12.1, y: 3.3 };
    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right2,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(i, d) = pair;

    println!("pair contains {:?} and {:?}", i, d);

    println!("rect area {:?}", rect_area(rectangle));
    println!("square area {:?}", rect_area(square(point, 2.2)));
}

fn rect_area(rectangle: Rectangle) -> f32{
    let Rectangle{top_left, bottom_right} = rectangle;
    let width = bottom_right.x - top_left.x;
    let height = bottom_right.y - top_left.y;

    width * height
}

fn square(point: Point, width: f32) -> Rectangle{
    let rectangle: Rectangle = Rectangle {
        top_left: Point {..point},
        bottom_right: Point{
            x: point.x + width,
            y: point.y + width
        }
    };
    rectangle
}

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

pub fn enums(){
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let mut x = Operations::Add;
    println!("x={:?}", x);
    x = Operations::Subtract;
    println!("x={:?}", x);

    let b = x.run(5, 3);
    println!("b={:?}", b);
}

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}


// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

//The most common place you'll see aliases is in impl blocks using the Self alias.
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

//lesson badly prepared 3.2.1
// An attribute to hide warnings for unused code.
//#![allow(dead_code)]

// enum Status {
//     Rich,
//     Poor,
// }

// enum Work {
//     Civilian,
//     Soldier,
// }

// fn use_test(){
//     // Explicitly `use` each name so they are available without
//     // manual scoping.
//     use crate::Status::{Poor, Rich};
//     // Automatically `use` each name inside `Work`.
//     use crate::Work::*;

//     // Equivalent to `Status::Poor`.
//     let status = Poor;
//     // Equivalent to `Work::Civilian`.
//     let work = Civilian;

//     match status {
//         // Note the lack of scoping because of the explicit `use` above.
//         Rich => println!("The rich have lots of money!"),
//         Poor => println!("The poor have no money..."),
//     }

//     match work {
//         // Note again the lack of scoping.
//         Civilian => println!("Civilians work!"),
//         Soldier  => println!("Soldiers fight!"),
//     }
// }

// enum with implicit discriminator (starts at 0)

//3.2.2 enum can also be used as C-like enums
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn c_like_enums(){
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("green #{:06x}", Color::Green as i32);
}
