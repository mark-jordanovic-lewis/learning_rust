extern crate reqwest;
mod fake_file_iterator;
use fake_file_iterator::Lines;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut word_count_map: HashMap<&str, i32> = HashMap::new();
    fake_file_iterator::read_all_lines_from_url("http://www.gutenberg.org/cache/epub/1661/pg1661.txt");
}

// make get request to url and build word count map
fn word_incidence_count<'a>(url: &'a str, map: &mut HashMap<&str, i32>) -> io::Result<()> {

    let mut res = reqwest::get(url);
    let mut lines = Lines::new(res);

    while let Some(Ok(line)) = lines.next() {
        for token in line.split(|c:char| ! c.is_alphabetic() ){
            let word = token.to_lowercase();
            // `map.entry(key).or_entry(default_insert)` returns the `value` in `key`
            // or creates `key => default_value` in the HashMap
            let mut count = map.entry(word).or_insert(0);
            *count += 1;
        }
    }
    Ok(())
}