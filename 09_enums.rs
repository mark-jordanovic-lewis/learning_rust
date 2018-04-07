fn main() {
    // Rust ENUM //
    // ========= //
    let start = Direction::Left;
    println!("Starting dir: {:?}", start);
    let mut d = start;
    for _ in 0..8 {
        println!("{:?}", d);
        d = d.next();
    }

    // C Style ENUM //
    // ============ //
    let s = Speed::Slow;
    print!("{:?} has a ", s);
    let speed = s as u32; // casting an enum to an integer value
    println!("speed of {:?}.", speed);

    // Hard Core Enum //
    // ============== //
    let n = Value::Float64(2.4);
    let b = Value::Boolean(true);
    let s = Value::Str("I r 1x stringz".to_string());
    eat_and_dump(n);
    eat_and_dump(b);
    eat_and_dump(s);
}

// ENUM //
// ==== //
// Enums cannot be compared as std, deriving the PartialEq trait allows it.
// PartialEq assumes that all fields implement it and build up a comparison from them.
// Like Debug it can be implemented manually.

// Direction is a very simple state machine
#[derive(Debug,PartialEq)]
enum Direction {
    Up, Down, Left, Right
}
impl Direction {
    fn to_s(&self) -> &'static str {
        match *self { // we have to dereference when matching!! So *self has type Direction
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right"

        }
    }
    fn next(&self) -> Self {
        use Direction::*;
        match *self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down
        }
    }
}

// C Style ENUM //
// ============ //
// increments to these enums occur in 1's. We only HAVE to give the first a value.
#[derive(Debug,PartialEq,PartialOrd)]
enum Speed {
    Slow = 10,     // different named Enum values are called `variants`
    Medium = 20,
    Fast = 50
}

#[derive(Debug)]
enum Difficulty {
    Easy = 1,
    Medium,       // = 2
    Challenging,  // = 3
    Hard,         // = 4
    Nightmare     // = 5
}

// Hard Core Enum //
// ============== //
// How to save different types in a type in a type safe way (lol)
#[derive(Debug)]
enum Value {
    Float64(f64),
    Str(String),
    Boolean(bool)
}

fn eat_and_dump(v: &Value) {
    use Value::*;
    match *v { // dereferenceing to get correct type
        Float64(n) => println!("This was a Float64 (f64) with value: {:?}", n),  // fine to copy the value for f64
        Str(ref s) => println!("This was a Str (String) with value: {:?}", s),   // String does not implement copy (and a good thing too - strings get big)
        Boolean(b) => println!("This was a Boolean (bool) with value: {:?}", b)  // fine to copy the value for bool
    }
}

// This could be used to build a ruby like syntax within Rust:
#[derive(Debug)]
enum Number {
    Float(f64),
    Integer(i32)
}
enum TrueClass {
    True = true
}
enum FalseClass {
    False = false
}
enum Array {
    // somehow wrangle vectors of one tuples in here.
}
// ...
