fn main() {

    // TUPLES //
    // ====== //
    println!("TUPLES\n=====");
    let tuple = add_mul(2.0, 3.4);
    println!("tuple: {:?}", tuple);
    // get values by index (location in tuple)
    println!("tuple.0: {}, tuple.1: {}", tuple.0, tuple.1);
    let (add, mul) = tuple;
    // extraction of values
    println!("add: {:?}, mul: {}", add, mul);
    // tuples can hold different types
    let tuple = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    // using the iterator method `enumerate` to generate a tuple in a for loop
    for (counter, element) in ["one", "two", "three"].iter().enumerate() {
        println!("{}: {:?}", counter, element);
    }
    let names = ["one", "ten", "hundred", "thousand"];
    let nums = [1,10,100,1000];
    for description in names.iter().zip(nums.iter()) {
        println!("{} = {:?}", description.0, description.1);
    }

    // STRUCTS //
    // ======= //
    println!("STRUCTS\n======");
    let p1 = Person::new("John", "Smith");
    println!("Person one's fullname: {}", p1.full_name());
    let mut p2 = p1.copy();
    p2.set_first_name("Adrian");
    println!("Person two's fullname: {}", p2.full_name());

    // Lifetimes of a struct containing a borrowed variable is an important concept to understand - stongly coupled to the concept of closure.
    // let a = A { a: "hello damnit!"};
    // println!("{:?}", a); // will throw an error
    let b = B { a: "hello damnit!"};
    println!("{:?}", b);
    let string = "I am a string instance in main".to_string();
    let c = C { c: &string };
    println!("{:?}", c);
    // let failing_c = makes_c;

    // TRAITS //
    // ====== //
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show implementation for: {}", s1);
    println!("show implementation for: {}", s2);
    // using the debug implementation on Person
    println!("p1 struct debug: {:?}", p1);
    // dumping Person to output through the generic function dump<T>
    dump(&p1);
}



// simple tuple returning function#
fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}



// Person Struct (they are like objects)
//#[derive(Debug)] // this causes the compiler to use the declaration of the struct as the debug output (implemented manually below as a trait)
struct Person {
    first_name: String,
    second_name: String
}
impl Person { // It's got a capital letter!
    fn new(first: &str, second: &str) -> Person {
        Person {
            first_name: first.to_string(),
            second_name: second.to_string()
        }
    }
    // self is passed implicitly on a method call
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.second_name)
    }
    // Self is shorthand for 'this struct I am implementing' (ie/ Self == Person)
    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.second_name)
    }
    // can update state with mutable self args from a mutable variable
    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }
    // data MOVES into a method when you have non-referential self
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.second_name)
    }
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

// Trying to make a struct which holds a slice, rather than a concrete type
// causes a `error[E0106]: missing lifetime specifier` on use (now throws a compile error here!)
// #[derive(Debug)]
// struct A {
//     a: &str  // error[E0106]: missing lifetime specifier
// }
// So we have to use the concept of `static` to declare that the concrete type will exist for the
// entire program runtime
#[derive(Debug)]
struct B {
    a: &'static str
}

// you can use static to return a static string slice from a function but this is very specific to strings.
// and will not work with slices of arrays or vectors.
fn how_many(i: i32) -> &'static str {
    match i {
        0 => "none",
        1 => "one",
        _ => "many"
    }
}

// We can define the lifetime of a slice borrowed from another instance by tying it to both the lifetime of
// the owning instance and the struct itself
#[derive(Debug)]
struct C <'live_at_least_as_long_as_the_C_struct> {
    c: &'live_at_least_as_long_as_the_C_struct str
}
// the C struct provides the idea of the lifetime of a borrowed value held by a struct, the solid instance of the value
// only lasts for the duration of the block that it is in, rendering the return of this function an error.
// fn makes_c() -> C <'static> {
//     let string = "I don't live outside this function closure".to_string();
//     C { c: &string} //error[E0597]: `string` does not live long enough
// }

// TRAITS // kinda like interfaces for Go.
// ====== //
// like forward declaration of a function in c++, traits can be implemented by any struct.
trait Show {
    fn show(&self) -> String;
}

// implement show for i32
impl Show for i32 {
    fn show(&self) -> String {
        format!("four byte signed int {}", self)
    }
}

// implement show for f64
impl Show for f64 {
    fn show(&self) -> String {
        format!("eight byte float {}", self)
    }
}

// Generics are based on traits. For a generic function which depends on some trait the passed value must
// implement that trait. For example `Debug`
fn dump<T>(value: &T)
where T: std::fmt::Debug {
    println!("Generically dumping out {:?}", value);
}
// One must be very specific with generics as the following exapmle demonstrates
fn sqr<T>(x: T) -> T::Output // there is no guarantee that `x` and `x * x` will have the same type, so the associated type T:Output must be declared
where T: std::ops::Mul + Copy { // Mul is declared as expected, but Copy (and maybe even more) triats must be declared.
    x * x
//  ^-- x is moved here and copied for the next access
}
