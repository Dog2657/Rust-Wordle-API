use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::Rng;
use std::sync::Mutex;

use chrono::Duration as Chrono_Duration;
use cron::Schedule;
use chrono::Utc;
use std::str::FromStr;

use std::thread;
use std::time::Duration;

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

static CURRENT_WORD: Mutex<Word> = Mutex::new(Word { value: String::new() });

#[get("/")]
fn get_current_word() -> String {
    return CURRENT_WORD.lock().unwrap().value.to_string();
}


#[launch]
fn rocket() -> _ {    
    CURRENT_WORD.lock().unwrap().value = get_random_word();

    thread::spawn(|| {
        let schedule = Schedule::from_str("0 0 0 * * *").unwrap();

        loop {
            let next = schedule.upcoming(Utc).next().unwrap();
            let next_local = next.with_timezone(&chrono::Local) - Chrono_Duration::hours(1);
            //You must change the "Chrono_Duration::hours(1)" to how far or behind utc your timezone is

            let now = chrono::Local::now();
            
            let duration = next_local - now;
            let std_duration = Duration::from_secs(duration.num_seconds() as u64);
            
            thread::sleep(std_duration);
            {//Code to be run
                CURRENT_WORD.lock().unwrap().value = get_random_word();
            }
            thread::sleep(Duration::from_secs(5));//1 second pause
        }
    });

    rocket::build()
        .mount("/", routes![get_current_word])
}