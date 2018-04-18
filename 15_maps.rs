// mod lib;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::io;

fn main() {
    // MAPS //
    // ==== //
    let de_en = [("one", "eine"), ("two", "zwei"), ("three", "drei")];
    // linear search through array
    if let Some(val) = de_en.iter().find(|t| t.0 == "two") {
        println!("I found two: {:?}", val);
    }

    // hash access of similar data structure to above
    let mut map = HashMap::new();
    map.insert("key_1", "val_1");
    map.insert("key_2", "val_2");
    map.insert("key_3", "val_3");
    println!("has 'key_2': {:?}, which is: {:?}", map.contains_key("key_2"), map.get("key_2"));

    // mutating values (have to work out a way of mutating `&str`s, if it is even possible
    let mut int_map = HashMap::new();
    println!("insert return: {:?}", int_map.insert("key_1", 1));
    println!("insert err return: {:?}", int_map.insert("key_1", 1));
    println!("entry return: {:?}", int_map.entry("key_1"));
    println!("entry err return: {:?}", int_map.entry("key_2"));
    int_map.insert("key_2", 2);
    int_map.insert("key_3", 3);
    println!("BEFORE MUTATE - has 'key_2': {:?}, which is: {:?}",int_map.contains_key("key_2"),int_map.get("key_2"));
    match int_map.get_mut("key_2") {
        Some(val) => *val = 12,
        None => panic!("Now we panic!")
    }
    println!("AFTER MUTATE - has 'key_2': {:?}, which is: {:?}",int_map.contains_key("key_2"),int_map.get("key_2"));

    // iterating over maps does NOT guarantee order is maintained from insertion order
    // - want to access map later so everything is borrowed
    for (key, val) in &int_map {
        println!("key: {}, val: {}", key, val);
    }
    println!("keys: {:?}", &int_map.keys());
    println!("values: {:?}", &int_map.values());

    // SETS //
    // ==== //
    // reasonable implementation
    let fruits = make_set("apple orange apricot peach pear apple pear peach grapes apricot");
    println!("fruit: {:?}", fruits);
    let colours = make_set("orange white blue green black yellow peach");
    println!("colour: {:?}", colours);
    // all set operations generate iterators thast must be `collected` into a HashSet (or whatever container)
    let fruit_that_is_colour = colours.intersection(&fruits);
    println!("fruit that is colour: {:?}", fruit_that_is_colour);
    // unreasonable implementation
    let fruits = make_string_set("apple orange apricot peach pear apple pear peach grapes apricot");
    println!("fruit: {:?}", fruits);
    let colours = make_string_set("orange white blue green black yellow peach");
    println!("colour: {:?}", colours);
    // the below line will collect the iterator into a HashSet<&String> (pointers to string literals - not string slices)
    let string_lit_pointer_hash = colours.intersection(&fruits).collect();
    let str_slice_hash = colours.intersection(&fruits).cloned().collect();
    println!("fruit that is colour: {:?}", fruit_that_is_colour);

}

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

// Having string literals in a collection is a recipe for borrow issues later on...
fn make_string_set(words: &str) -> HashSet<String> {
    words.split_whitespace.map(|s: &str| s.to_string() ).collect()
}