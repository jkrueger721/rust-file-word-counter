use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    file_read(&args[1]);
}

fn file_read(path: &String) {
    assert!(Path::new(&path).exists());
    let file = File::open(path).expect("no file in path");
    let mut reader = BufReader::new(file);
    let mut file_contents = String::new();

    reader.read_to_string(&mut file_contents);
    let collect = file_contents.split_whitespace().collect::<Vec<&str>>();

    let mut word_counter: HashMap<&str, i32> = HashMap::new();

    for w in collect {
        *word_counter.entry(w).or_insert(0) += 1;
    }

    for (word, occurrances) in word_counter {
        println!("File contains: {occurrances} of {word}");
    }
}
