use std::collections::HashMap;
use std::fs;
use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    word_count_in_dir();
}


// using ? and .expect("") requires us to return a Result<type, &str>
fn word_count_in_dir() -> io::Result<()> {

    // loop over entries in dir
    for entry in fs::read_dir("./")? {

        // get some details about the file path
        let entry = entry? ;
        let path = entry.path();
        if !path.is_file() { continue }                         // goto main loop top if entry not a file

        // if the file has an extension
        if let Some(ext) = path.extension() {
            if !(ext == "rs") { continue }                      // goto main loop top  if not a rust file

            // get the text from the open file
            let mut file = File::open(&path).expect("Cannot open file");
            let mut text = String::new();
            file.read_to_string(&mut text).expect("Could not read from file.");

            // build the hash map from the file
            let mut hash = HashMap::new();
            let skip: Vec<&str> = vec![" ", "\n", "\t", ""];
            for s in text.split(|c: char| !c.is_alphabetic()) {
                if skip.contains(&s) { continue }               // goto read loop top if token uninteresting
                let word = s.to_lowercase();
                let mut count = hash.entry(word).or_insert(0);
                *count += 1;
            }

            // collect HashMap into a vector and sort high -> low
            let mut word_count: Vec<(String, i32)> = hash.into_iter().collect();
            word_count.sort_by(|a: &(String, i32), b: &(String, i32)| b.1.cmp(&a.1));

            // output results
            let mut under: String = "==============".to_string();
            for _ in 0..entry.path().display().to_string().len() {
                under += "=";
            }
            println!("word count in {}", entry.path().display());
            println!("{}", under);
            for (i, entry) in word_count.iter().take(10).enumerate() {
                println!("{}: {} occurs {} times.", i+1, entry.0, entry.1);
            }
            println!();
        }
    }
    Ok(())
}

