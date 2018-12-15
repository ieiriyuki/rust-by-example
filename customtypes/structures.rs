use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

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
    p1: Point,
    p2: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { p1: Point { x: p1_x, y: p1_y },
                    p2: Point { x: p2_x, y: p2_y } } = rectangle;
    let area = (p1_x - p2_x).abs() * (p1_y - p2_y).abs();

    area
}

fn square(corner: Point, length: f32) -> Rectangle {
    let another_corner = Point{ x: corner.x + length,
                                y: corner.y + length };
    let rectangle = Rectangle{ p1: corner,
                               p2: another_corner };

    rectangle
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "a corner is( {0}, {1} )", self.p1.x, self.p1.y);
        write!(f, "another corner is ( {0}, {1} )", self.p2.x, self.p2.y)
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let _rectangle2 = Rectangle {
        p1: Point { x: 0.1, y: 0.1 },
        p2: Point { x: 0.3, y: 0.4 },
    };

    let an_area = rect_area(_rectangle2);
    println!("an area size is {:?}", an_area);

    let another_area = square(Point { x: 0.1, y: 0.1 }, 0.5);
    println!("another area is {}", another_area)
}
