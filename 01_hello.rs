fn main() {

    // immutable assignment and string interpolation

    let answer = "world!";
    println!("Hello, {}", answer);

    let assertion = 42;
    //assert_eq!(assertion, 40); // throws runtime err
    assert_eq!(assertion, 42); // produces no output


    // loops ranges and conditionals

    for i in 0..5 { // ranges are non inclusive
        if i % 2 == 0 {
            println!("I am count number {} and I am even", i);
        } else {
            println!("I am count number {} and I am odd", i);
        }
    }

    // there are no ternaries :'(
    for i in 5..10 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("I am count number {} and I am {}", i, even_odd);
    }


    // mutable assignment

    let mut sum = 0; // dynamic typing makes this an i32
    for i in 0..5 {
        sum += i;
    }
    println!("I summed i to {}", sum);


    // casting to type

    let mut new_sum = 0.0; // dynamic typing makes this an f32
    for i in 0..5 {
        new_sum += i as f64; // casting the i32 i to a f64
    }
    println!("I summed i to {}", new_sum);


    // calling functions

    let var = -2.4;
    let sqr_answer = sqr(var);
    println!("I squared 2.4 and got: {}", sqr_answer);
    println!("absolute val of {} is {}", var, abs(var));
    println!("factorial 5 is {}", recursion_factorial(5));


    // passing vars and modifying by reference

    let a = 1;
    println!("Pass by reference: I have a = {}, pass_by_ref(&a) = {}, and a still = {}", a, pass_by_ref(&a), a);
    let mut b = 1;
    println!("Pass by reference: I have b = {}, now I call modify_by_ref(&mut b)", b);
    modify_by_ref(&mut b);
    println!(" .....................and b = {}", b);


    

}

// declaring a sqr function - arg and return types are explicit, return of last line is implicit and no ; req'd
fn sqr(x: f64) -> f64 {
    x * x
}
fn abs(x: f64) -> f64 {
    if x > 0.0 { x } else { -x }
}
fn recursion_factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * recursion_factorial(n-1) }
}

// function with argument passing by reference - must declare as reference using '&' and dereference to use using '*'
fn pass_by_ref(x: &i32) -> i32 {
    *x + 1
}
// function with mutable reference argument
fn modify_by_ref(x: &mut i32) {
    *x += 1
}
