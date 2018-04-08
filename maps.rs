mod lib;
extern crate reqwest;
use std::collections::HashMap;
use std::io::Read;
use std::io;

fn main() {
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
    int_map.insert("key_1", 1);
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
}

