// all this will throw errors


// EXAMPLE ONE: assignment
fn move_by_reassignment_err() {
    let s1 = "hello dolly".to_string();
    // this line moves the value in s1 into s2
    let s2 = s1;
    // this line will throw a `use of moved value` err
    println!("s1 {}", s1);
}
// You can avoid the move error in example one using clone:
fn fixed_move_by_reassignment_err() {
    let s1 = "hello dolly".to_string();
    // this line moves the value in s1 into s2
    let s2 = s1.clone();
    // this line will throw a `use of moved value` err
    println!("s1 {}", s1);
}


// EXAMPLE TWO: indirect assignment
fn dump(s: String) {
    println!("{}", s);
}
fn move_by_pass_by_move_error() {
    let s1 = "hello dolly".to_string();
    // this line moves the value into the function argument (indirect assignment)
    dump(s1);
    // this line will throw a `use of moved value` err
    println!("s1 {}", s1);
}
// you can avoid the move error in example two by passing by reference:
fn dump(s: &str){ // or, worse: fn dump(s: &String) {
    println!("{}", s);
}
fn main() {
    let s1 = "hello dolly".to_string();
    dump(&s1);
    println!("s1 {}", s1);
}

// EXAMPLE THREE: you know what a closure is dude.
{
let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a,b and c are visible
    }
    // the string c is dropped
    // a,b are visible
    for i in 0..a {
        let b = &b[1..];
        // original b is no longer visible - it is shadowed.
    }
    // the slice b is dropped
    // i is _not_ visible!
    let s1 = "hello dolly".to_string();
    let mut rs1 = &s1;
    {
        let tmp = "hello world".to_string();
        rs1 = &tmp;
    }
    // tmp is out of scope (and is dropped from the heap) so the reference is also gone
    //   - this is a compile error in rust, thankfully.
    println!("ref {}", rs1);
}
