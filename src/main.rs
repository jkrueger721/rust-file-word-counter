use async_std::{fs::File, io, prelude::*, task};
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader_task = task::spawn(async move {
        file_read(&args[1]).await;
    });
    task::block_on(reader_task);
}

async fn file_read(path: &String) -> io::Result<String> {
    assert!(Path::new(&path).exists());
    let mut file = File::open(path).await?;
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).await?;
    let collect = file_contents.split_whitespace().collect::<Vec<&str>>();

    let mut word_counter: HashMap<&str, i32> = HashMap::new();

    for w in collect {
        *word_counter.entry(w).or_insert(0) += 1;
    }

    for (word, occurrances) in word_counter {
        println!("File contains: {occurrances} of {word}");
    }
    Ok((file_contents))
}
