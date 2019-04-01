use std::fmt;

const PI : f64 = 3.141592654;

enum Direction {
    Right,
    Up,
    Down,
    Left
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Right    => write!(f, "Right"),
            Direction::Up       => write!(f, "Up"   ),
            Direction::Down     => write!(f, "Down" ),
            Direction::Left     => write!(f, "Left" ),
        }
    }
}

pub fn main() {
    let a : i32 = 70000;
    let b : f32 = 0.2;
    println!("a = {}", a);
    println!("b = {}", b);

    let mut c = '😕';
    println!("c = {}", c);
    c = 'A';
    // c = 12; // compile error due to type violation
    println!("c = {}", c);

    let d = true;
    println!("d = {}", d);

    let e = Direction::Up;
    println!("direction is {}", e);

    println!("Happy {} day!", PI);

    let (zero, one) = (0, 1);
    let binary = (zero, one);
    println!("Binary: ({}, {})", binary.0, binary.1);
    let ali : (&str, f32, (i32, i32), (i32, i32, (i32, i32))) = ("ali", 3.50, binary, (zero, one, binary));
    let (name, gpa, _, _) = ali;
    println!("name: {}, gpa: {}", name, gpa);
}
