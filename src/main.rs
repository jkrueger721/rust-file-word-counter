use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn main() {
    file_read();
}

fn file_read() {
    let file = File::open("test.txt").expect("no file in path");
    let mut reader = BufReader::new(file);
    let mut file_contents = String::new();
    let collect = file_contents.split_whitespace().collect::<Vec<&str>>();

    let mut word_counter: HashMap<&str, i32> = HashMap::new();

    for w in collect {
        *word_counter.entry(w).or_insert(0) += 1;
    }

    for (word, occurrances) in word_counter {
        println!("File contains: {occurrances} of {word}");
    }
}
