// including and using modules
mod float_step_iterator;
use float_step_iterator::range;

fn main() {
    // simple closures in rust are like JS functions or lambdas:
    let x_sqd = |x| x * x;
    // compile err: error[E0277]: the trait bound `[closure@closures.rs:3:17: 3:26]: std::fmt::Debug` is not satisfied
    // println!("{:?}", x_sqd);
    println!("x_sqd(2) = {:?}", x_sqd(2));
    // compile err: error[E0308]: mismatched types, expected integral variable, found floating-point variable
    // println!("x_sqd(2.0) = {:?}", x_sqd(2.0)); // the first call to the closure fixes the type
    let m = 2.0;
    let c = 3.5;
    let linear_curve_closure = |x| m*x + c;
    // closures BORROW values from the enclosing block!
    // The lifetime of a closure is dependent on the lifetime of it's borrowed variables.
    println!("linear_curve_closure(2.0) = {:?}", linear_curve_closure(2.0));
    // apply defines the generic constraint of the closure (closure dependent)
    println!("apply(2.0, linear_curve_closure) = {:?}", apply(2.0, linear_curve_closure));
    // the linear_curve_closure value has now been consumed by the apply function:
    // compile error: error[E0382]: use of moved value: `linear_curve_closure`
    // println!("linear_curve_closure(2.0) = {:?}", linear_curve_closure(2.0));
    // we can also pass a reference to a closure:
    let linear_curve_closure_2 = |x| m*x + c;
    println!("apply_2(2.0, linear_curve_closure_2) = {:?}", apply_2(2.0, &linear_curve_closure_2));
    // we can use a closure to mutate a captured reference but not a passed one:
    // Because "let mut mutable_string" has type &str. "let mut mutable_string: &str" is equivalent
    // so then we already know that the mutable_string variable is a mutable variable, but the value it holds is a &str, which is a string slice of immutable string data
    // so we can change which string slice we point to, but not change the string data through this reference
    let mut mutable_str_ref = "I am a mutable string";
    let doesnt_mutate_string = |mut s| s = "s was not mutated to this"; // this returns ()!!
    let returned_string = doesnt_mutate_string(&mut mutable_str_ref);
    println!("returned_string is: {:?}", returned_string);
    println!("mutable string is still: {:?}", mutable_str_ref);
    // closure does a mutable borrow of &str
    // let mut doesnt_mutate_string_and_throws_error = || mutable_str_ref = "s was also not mutated to this."; // and other attempts
    // does_mutate_string_and_throws_error();
    // println does a mutable borrow of string
    // println!("mutable string is now: {:?}", mutable_str_ref);
    mutate_closure_applier(|| mutable_str_ref = "s was mutated to this.");
    println!("mutable string is now: {:?}", mutable_str_ref);
    // we can also actually mutate a string but we can't get teh value back as the closure has borrowed it:
    let mut string = "mutable string".to_string();
    // closure does a mutable borrow of string
    let mut string_mutator = || string = "mutated string".to_string();
    string_mutator();
    // println does a mutable borrow of string
    // string is gone forever into the closure and the closure is... closed.
    // println!("strign after mutation: {:?}", string);
    // BUT we can put the closure and it's application in a block, freeing up the variable again, even after its mutation!
    let mut s = "world";
    {
        let mut changer = || s = "chicken";
        changer();
    }
    println!("Is the string mutated? {:?}", assert_eq!(s, "chicken") == ());
    // we can move variables into closures if we so wish (this covers all the variables taken from the context)
    // moved closures can be called outside of the lifetime of the moved variables (they now belong to the closure, not the original context)
    let name = "dolly".to_string();
    let age = 42;
    let c = move || {
        println!("name {} age {}", name,age);
    };
    c();
    // println!("name {}", name);
    println!("age {:?}", age); // this is ok because i32s do not behave like strings, they have Copy implemented
    // closures are nice with iterators, no temporary objects are created during this line:
    let sum: f64 = range(0.0,1.0,0.1).map(|x| x.sin()).sum();
    println!("sin sum = {:?}", sum);
    let tuples = [(10,"ten"),(20,"twenty"),(30,"thirty"),(40,"forty")];
    let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.1);
    for name in iter {
        println!("{} ", name);
    }
}

// A CLOSURE is a STRUCT under the hood. linear_curve_closure is really this, where m and c are passed
// in implicitely as a `linear_curve_closure::new(m, c)` when the function is called
struct linear_curve_struct<'value_lifetime> {
    m: &'value_lifetime f64,
    c: &'value_lifetime f64
}
// this definition reads:
// this implementation uses one parameter, 'value_lifetime, these methods will be implemented for
// the linear_curve_struct type which is parameterised over 'value_lifetime
impl<'value_lifetime>  linear_curve_struct<'value_lifetime> {
    fn call(&self, x: f64) -> f64 { // this is known as 'implementing the call operator'
        self.m*x + self.c
    }
}
// the generic constraint of linear_curve_closure is this:
fn apply<F>(x: f64, f: F) -> f64
where F: Fn(f64) -> f64 {
    f(x)
}
// the generic constraint of linear_curve_closure is this:
fn apply_2<F>(x: f64, f: &F) -> f64
where F: Fn(f64) -> f64 {
    f(x)
}
// the mutable string function applier - note how this does not work in the 'normal closure' case.
fn mutate_closure_applier<F>(mut f: F)
where F: FnMut() { // where F implements the FnMutate trait
    f()
}
