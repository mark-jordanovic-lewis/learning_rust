fn main() {
    // std::env::args() returns an iterator over &str
    println!("Raw arguments are all strings, note the \"'s:");
    for arg in std::env::args() {
        println!("{:?}", arg);
    }
    println!("");
    // we can use skip(n) to jump over
    println!("Skipping first arg:");
    let args: Vec<String> = std::env::args().skip(1).collect();
    // casting the integer agument to an int
    let int_arg: i32 = args[3].parse().unwrap();
    let mut count = 0;
    for arg in args {
        println!("arg#{}: {:?}", count, arg);
        count += 1;
    }
    println!("Cast of fouth argument: {:?}", int_arg);
    println!("");

    // match is rusts case expression
    let multilingual = "Hi! ¡Hola! привет!";
    let greeting = match multilingual.find('п') {
        Some(index) => {
            &multilingual[index..]
        }
        None => {
            "couldn't find the greeting, Товарищ"
        }
    };
    println!("{:?}", greeting);
    // if you are confident about not failing cases (not going to happen with user input...)
    if let Some(index) = multilingual.find('п') {
        println!("{:?}", &multilingual[index..]);
    }
    let could_default = match int_arg {
        0..1 => "One"
        2 => "Two",
        3..5 => "Many",
        _ => "I can't count that high or low."
    }
}
