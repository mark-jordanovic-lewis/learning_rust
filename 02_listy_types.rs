// use is rusts include/require/import
use std::f64::consts;


fn main() {
    // there are a bunch of handy constants
    let theta = 2.0 * consts::PI;
    let abs_difference = (theta.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);


    // arrays - can only contain one type

    let arr = [10,20,30,40,50]; // this has type [i32; 5] (the size is part of the type)
    println!("arr ahs to be debug printed and comes out like: arr = {:?}", arr);
    for i in 0..5 {
        println!("arr[{}] = {}", i, arr[i]);
    }
    println!("arr has length: {}", arr.len());


    // slices - views onto underlying array
    let beginning_slice = &arr[0..3]; // data is never copied into slices, they are pointer arrays, but handled really nicely.
    let ending_slice = &arr[3..5];    // in rust this is called 'borrowing' a borrowed value always belongs to the wner who passed it.
    println!("beginning_slice: {:?}, ending_slice: {:?}", beginning_slice, ending_slice);
    println!("arr sums to: {}", sum_slice(&arr)); // look how we have dereferenced this for passing as a slice
    // println!("this will throw a runtime err. Don't try to access elements outside of the array {}", ending_slice[5]); // this will throw a runtime error (panic)


    // slice has a `get` method which returns an `Option` ie/ `Just <val>` or `None` - welcome on in MONADs

    let some_val = ending_slice.get(0);
    let none = ending_slice.get(5);
    println!("some_val");
    println!("  ending_slice.get(0): {:?}", some_val);
    println!("  is_none: {}, is_some: {}", some_val.is_none(), some_val.is_some()); // loking to see if values are contained
    println!("  unwrapped: {}", some_val.unwrap()); // note that this unwraps a memory address
    println!("none");
    println!("  ending_slice.get(5): {:?}", none);
    println!("  is_none: {}, is_some: {}", none.is_none(), none.is_some());
    // println!("  unwrapped: {}", none.unwrap()); // note that this throws an error (null pointer)
    // let unwrapped_none = if none.is_some() { *none.unwrap() } else { -1 }; // can unwrap conditionally, must assign default though.
    let unwrapped_none = *ending_slice.get(5).unwrap_or(&-1);                 // shorthand for the above line
    println!("The value of none is {}.", unwrapped_none);


    // Vectors - extensible slices, push onto and acces like array

    let mut vector = Vec::new();
    vector.push(10);
    vector.push(20);
    vector.push(30);
    println!("vector: {:?}", vector);
    println!("vector[0] = {}", vector[0]);
    println!("vector[0] = {:?}", vector.get(0));
    // println!("vector[3] = ", vector[3]);        // throws runtime error
    println!("vector.get(3) = {:?}", vector.get(3));
    dump_array(&vector); // calling a func with vector arg


    // vectors act as right stacks with push and pop and have an easy instantiation operator
    let mut vect = vec![0];
    println!("initial vect:      {:?}", vect);
    vect.push(1);
    println!("vect after push:   {:?}", vect);
    vect.pop();
    println!("vect after pop:    {:?}", vect);
    // nice vector operations can be used to add and remove
    vect.extend(1..5); // you can give any iterator of correct type to extend
    println!("vect after extend: {:?}", vect);
    vect.insert(3, 400);
    println!("vect after insert: {:?}", vect);
    vect.remove(3);
    println!("vect after remove: {:?}", vect);
    // they can be sorted and uniqued
    let mut vect2 = vec![1,4,2,5,7,4,3,3,5,6,2,1,3];
    println!("vect on init:      {:?}", vect2);
    let mut vect3 = vect2.clone();
    vect2.sort();
    println!("vect after sort:   {:?}", vect2);
    vect2.dedup();
    println!("vect after dedup:  {:?}", vect2);
    println!("cloned vect before sort and dedup:  {:?}", vect3);
    vect3.dedup(); // dedup only removes consecutive duplicates
    println!("cloned vect after dedup:            {:?}", vect3);


    // Iterators - 'object's with a next method which returns an Option. As long as that value is not None, we keep calling next

    let mut iter = 0..4;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);

    println!("array iteration");
    for i in arr.iter() { // iter() acceses values in the the array
        println!("i = {}", i);
    }
    println!("vector iteration");
    for i in vector { // vectors and slices are implicitelyu converted to iter
        println!("i = {}", i);
    }

    // idiomatic use of iterators yusing the sum method (this is kinda functional)
    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);
    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);

    // the windows iterator makes overlapping views onto a slice
    println!("windows onto arr slice");
    let whole_slice = &arr;
    for window in whole_slice.windows(2) {
        println!("2-window: {:?}", window);
    }
    for window in whole_slice.windows(3) {
        println!("3-window: {:?}", window);
    }
    // the chunks iterator gives non-overlapping views on
    println!("chunks onto arr slice");
    let whole_slice = &arr;
    for chunk in whole_slice.chunks(2) {
        println!("2-chunk: {:?}", chunk);
    }
    for chunk in whole_slice.chunks(3) {
        println!("3-chunk: {:?}", chunk);
    }

    let mut string_vector: Vec<String> = ["one".to_string(), "two".to_string(), "three".to_string()].to_vec();
    let comparison_vect = string_vector.clone();
    // three iterator types correspond to the three argument types
    for s in string_vector.iter() { println!("{:?}", s); } // for s in &vec {...} // s is a &String
    // I am cloning here so I can reassign the vecotr content.
    for (i, s) in string_vector.clone().iter_mut().enumerate() { // for s in &mut vec {...} // s is a &mut String
        s.push_str(" mutated");
        string_vector[i] = format!("i: {:?}, s: {}", i, s.to_string());
    }
    // this consumes the vector so it is no longer available
    let vect = string_vector.clone();
    for s in string_vector.into_iter() { println!("{:?}", s); } // for s in vec {...} // s is a String
    // have to use cloned vect as string_vector was consumed in above loop
    for n in vect.iter().map(|x: &String| x.len()) { println!("length {}", n); }// n is usize
    for s in vect.iter().filter(|x: &&String| x.contains("three")) { println!("{:?} contains three", s); } // s is &String
    // for ensuring comparison works correctly we need to ensure the types are the same
    for s in comparison_vect.iter().filter(|x: &&String| *x == "one") {
        assert_eq!(s, "one");
        println!("Should see this three times");
    }
    // same as
    for s in comparison_vect.iter().filter(|x| *x == "one") {
        assert_eq!(s, "one");
        println!("Should see this three times");
    }
    // same as
    for s in comparison_vect.iter().filter(|&x| x == "one") {
        assert_eq!(s, "one");
        println!("Should see this three times");
    }
}

// function that takes a slice
fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..slice.len() {
        sum += slice[i]
    }
    sum
}

// vectors are passed the same way as slices
fn dump_array(x: &[i32]) {
    println!("arr is {:?}", x)
}
