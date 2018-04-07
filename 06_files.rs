use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::io::prelude::*;

fn main() {
    { // file reading block
        // files are closed at the end of the owning closure
        // these lines will not exit the program as they 'expect' a possible error to occur
        let first = env::args().nth(1).expect("Please supply a file name."); // nth will get the nth arg from the iterator
        let mut file = File::open(&first).expect("Cannot open file.");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Cannot read from file.");
        println!("{} had       {:?} bytes", first, text.len());

        // use of the err returning functions
        let content = read_file_to_string(&first).expect("this was a bad file.");
        println!("{} still had {:?} bytes", first, content.len());
    }
    { // file writing block
        write_to_file("06_files-write_test.txt").expect("File writing failed");
    }
}

// this performs the entire above task and returns either the Ok(filecontent) or an error
fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => Ok(string),
        Err(e) => Err(e)
    }
}

// the shorthand for the function above
//   - io::Result<type> is shorthand for Result<type, io::Err>
//   - ? is shorthand for the entire match statement above
fn read_file_to_string_short(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

// a new string is allocated for each line read using the reader
fn read_all_lines_allocate_string(filename: &str) -> io::Result<()> {
    let file = File::open(&filename);
    let reader = io::BufReader::new(file);
    // can collect `lines()` into a vector or `enumerate()` as `reader.lines()` is an iterator
    for line in reader.lines() {
        let line = line?; // line taken `lines` is an io::Result<String> so we unwrap with `?`
        println!("{:?}", line);
    }
    Ok(())
}

fn read_all_lines_to_single_buffer(filename: &str) -> io::Result<()> {
    let file = File::open(&filename);
    let reader = io::BufReader::new(file);
    // allocate a single string to act as our buffer
    let mut buffer = String::new();
    while reader.read_line(&mut buffer)? > 0 {
        { // using this block to control the lifetime of the borrow from buffer
            let line = buffer.trim_right(); // line is borrowed from buffer and must die before we clear buffer
            println!("{:?}", line);
        }
        buffer.clear(); // memory allocated to buffer is not emptied until the block is exited.
    }
}

// writing out to files is much simpler than reading - but this in unbuffered
// this means that `write!` directly sends to the OS rather than buffering all the o/p until the file is closed.
// io::BufWriter is available to improve performance.
fn write_to_file(filename: &str) -> io::Result<()> {
    let mut out_file = File::create(filename)?;
    write!(out_file, "This is how to write to a file. {} I am sure there are more modes available.", 42);
    Ok(())
}
