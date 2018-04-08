extern crate reqwest;
use std::io;
use std::fs::File;
use std::io::prelude::*;

// R is some type which implements Read
pub struct Lines<R> {
    reader: io::BufReader<R>,
    buffer: String
}

// we have to implement new and next to simulate the behaviour of an iterator through the line of a file.
impl <R: Read> Lines<R> {
    pub fn new(r: R) -> Lines<R> {
        Lines { reader: io::BufReader::new(r), buffer: String::new() }
    }
    // cannot return borrowed stings without specifying lifetime.
    pub fn next<'whileSelfLives>(&'whileSelfLives mut self) -> Option<io::Result<&'whileSelfLives str>> { // either return None, Some(String) or Some(Err)
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(n_bytes) => if n_bytes == 0 {
                None
            } else {
                let line = self.buffer.trim_right();
                Some(Ok(line))
            }
            Err(e) => Some(Err(e))
        }
    }
}

pub fn read_all_lines_from_file(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;
    let mut lines = Lines::new(file);

    //    let Some(Ok(line)) = lines.next() // this will fail silently on err
    while let Some(line) = lines.next() {
        let line = line?;                   // this allows for read errors to be caught
        println!("{}", line);
    }
    Ok(())
}

pub fn read_all_lines_from_url(url: &str) -> io::Result<()> {
    let mut res = match reqwest::get(url) {
        Ok(response) => response,
        Err(err) => panic!("{:?}", err)
    };
    let mut lines = Lines::new(res);
    while let Some(line) = lines.next() {
        let line = line?;                   // this allows for read errors to be caught
        println!("{}", line);
    }
    Ok(())
}