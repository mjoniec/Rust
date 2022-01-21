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
#[allow(dead_code)]
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