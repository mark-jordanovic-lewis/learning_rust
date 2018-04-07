fn main() {
    let text = "text";         // this is a string slice  &str == &[u8]
    let s = text.to_string();  // this is a string        String == Vec<u8>
    dump_string_slice(text);
    dump_string_slice(&s);     // cast string into &str (string slice)

    let mut string = String::new(); // initially empty
    string.push('H');
    string.push_str("ello");
    string += ", World!!";
    string.pop();

    assert_eq!(string, "Hello, World!");
    string += " I am making a long string to play with later, I hope you don't mind.";

    let arr = array_to_string(&[10,20,30]);
    let res = format!("I turned array into a string so it could be printed: {}!", arr);
    println!("{}",res);
    // format! takes args and converts them into a string
    let arr_string = format!("{} {:?}", 'a', [10,20,30]);
    assert_eq!("a [10, 20, 30]", arr_string);

    // taking chunks out of strings
    let res_chunk = &res[21..27];
    println!("This was chunked out of res: {}", res_chunk);

    println!("");
    println!("multilingual chars:");
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch);
    }
    // rust chars are 4 byte Unicode elements, so the length and char count do not match
    println!("");
    println!("length:     {}", multilingual.len());
    println!("char count: {}", multilingual.chars().count());

    let maybe_found = multilingual.find('п');
    // remebmer the (None) and (Some value)
    if maybe_found.is_some() {
        let russian_hi = &multilingual[maybe_found.unwrap()..];
        println!("Russian hi: {}", russian_hi);
    }
    println!("Spanish hi: {}", spanish_banged(multilingual));
    // split_whitespace() returns an iterator, which we can .collect() into a Vec<&str> or just use as an iterator
    let multilingual_iter = multilingual.split_whitespace();
    // Have to be specific about what is collect()ed from an iterator for the compiler
    let mut multilingual_arr: Vec<&str> = multilingual_iter.collect();
    println!("iterating over: {:?}", multilingual_arr);
    let mut count = 0;
    for word in multilingual_arr {
        println!("{}: {:?}", count, word);
        count += 1;
    }
    // can extend a mutable vector using an iterator - this throws an error: error[E0382]: use of moved value: `multilingual_arr`
    //    multilingual_arr.extend(string.split_whitespace());
    //    println!("extended multilingual_arr: {:?}", multilingual_arr);

    // can coerce collect into a string (helping the compiler to decide how to cast the return value)
    let ws_stripped: String = string
        .chars()
        .filter(|chr| ! chr.is_whitespace() )
        .collect();
    println!("{} -> {:?}", string, ws_stripped);

    // I am up to Parsing with Nom
}


// string slice printer
fn dump_string_slice(s: &str) {
    println!("str '{}'", s)
}

// using to_string to convert non-strings to strings
fn array_to_string(arr: &[i32]) -> String {
    let mut res = String::new();
    res += &'['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push_str(", ");
    }
    res.pop();
    res.pop();
    res += &']'.to_string();
    res
}

// becuase of this we need to find the correct places to slice the array from
fn spanish_banged(input: &str) -> String {
    let ubang_index = input.find('¡');
    if ubang_index.is_some() {
        between_the_bangs(&input[ubang_index.unwrap()..]).to_string()
    } else {
        "Could not find a '¡' anywhere!".to_string()
    }
}

fn between_the_bangs(string: &str) -> String {
    let bang_index = string.find('!');
    if bang_index.is_some() {
        (&string[0..bang_index.unwrap()+1]).to_string()
    } else {
        "Could not find a '!' after a '¡'".to_string()
    }
}
