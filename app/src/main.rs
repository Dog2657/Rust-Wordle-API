use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::Rng;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use chrono::prelude::*;

#[macro_use] extern crate rocket;

struct Word {
    value: String,
}


fn get_word_from_index(index: i32) -> String{
    let file = File::open("words.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .nth( index as usize )
        .unwrap_or_else(|| Ok(String::new()))
        .expect("Error reading line")
}

fn get_total_words() -> i32 {
    let file = BufReader::new(File::open("words.txt").expect("Unable to open file"));
    let mut count  = 0;
    
    for _ in file.lines() {
        count = count + 1;
    }
    count
}

fn get_random_word() -> String {
    let total_words = get_total_words();
    let random_index = rand::thread_rng().gen_range(0..(total_words+1));
    let random_word = get_word_from_index(random_index);

    random_word
}

fn get_time_seconds() -> u32{
    let local_time = Local::now();
    (local_time.hour() * 3600) + (local_time.minute() * 60) + local_time.second()
}


static CURRENT_WORD: Mutex<Word> = Mutex::new(Word { value: String::new() });

#[get("/")]
fn get_current_word() -> String {
    return CURRENT_WORD.lock().unwrap().value.to_string();
}

#[launch]
fn rocket() -> _ {    
    CURRENT_WORD.lock().unwrap().value = get_random_word();

    thread::spawn(|| {
        loop {
            CURRENT_WORD.lock().unwrap().value = get_random_word();
            thread::sleep( Duration::from_secs(60) );
            thread::sleep( Duration::from_secs( 86400 - (get_time_seconds() as u64) ) );
        }
    });


    rocket::build()
        .mount("/", routes![get_current_word])
}
