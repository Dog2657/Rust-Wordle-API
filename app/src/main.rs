use std::io::{BufReader, BufRead};
use rand::seq::SliceRandom;
use chrono::Datelike;
use chrono::TimeZone;
use rand::thread_rng;
use std::io::Write;
use std::fs::File;
use std::thread;
use std::fs;

#[macro_use] extern crate rocket;


struct Date{
    day: u32,
    month: u32,
    year: i32
}


fn get_current_date() -> Date {
    let current_date = chrono::Utc::now();

    Date {
        day: current_date.day(), 
        month: current_date.month(),
        year: current_date.year(),
    }
}

fn get_date_from_string(string: &str) -> Date{
    let split: Vec<&str> = string
    .split("-")
    .collect();


    Date {
        day: split[0].parse::<u32>().unwrap(), 
        month: split[1].parse::<u32>().unwrap(),
        year: split[2].parse::<i32>().unwrap(),
    }
}

fn make_shuffle_file(){
    //Create temp folder (if dosn't exist)
    fs::create_dir_all("./.temp").unwrap();

    //Create shuffle file
    let mut file = File::create("./.temp/shuffle").unwrap();

    //-----------------------------//

    let Date {day, month, year} = get_current_date();

    let date_string = format!("{day}-{month}-{year}");

    file.write_all(format!("{}\n\n", date_string).as_bytes()).unwrap();


    //-----------------------------//

    //Get words file
    let words_file = File::open("words.txt").expect("file not found!");
    let reader = BufReader::new(words_file);

    //Load all words into Vector
    let mut lines: Vec<String> = reader.lines()
    .collect::<Result<_, _>>().expect("Vector error");

    //Randomize vector array
    let mut rng = thread_rng();
    lines.shuffle(&mut rng);

    let all_lines = lines.join("\n");
    file.write_all(all_lines.as_bytes()).unwrap();
}


#[get("/")]
fn get_current_word() -> String{
    let file = match File::open("./.temp/shuffle") {
        Ok(f) => f,
        Err(_) => {
            make_shuffle_file();
            match File::open("./.temp/shuffle") {
                Ok(f) => f,
                Err(err) => panic!("Unable to open file: {:?}", err),
            }
        }
    };

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let today = get_current_date(); 
    let start_date: Date = match lines.nth(0){
        Some(line) => get_date_from_string(&line.unwrap()),
        None => {
            make_shuffle_file();

            let file = File::open("./.temp/shuffle").expect("file not found!");
            let date = BufReader::new(file)
                .lines()
                .nth(0)
                .expect("unable to read new file")
                .unwrap();

            get_date_from_string(&date)
        }
    };
    

    let start_utc_date = chrono::Utc.with_ymd_and_hms(start_date.year, start_date.month, start_date.day, 0, 0, 0).unwrap();
    let today_utc_date = chrono::Utc.with_ymd_and_hms(today.year, today.month, today.day, 0, 0, 0).unwrap();

    let diff = today_utc_date - start_utc_date;

    let word = match lines.nth(diff.num_days() as usize + 1) {
        Some(line) => line.unwrap(),
        None => {
            make_shuffle_file();

            let file = File::open("./.temp/shuffle").expect("file not found!");
            BufReader::new(file)
                .lines()
                .nth(2)
                .expect("unable to read new file")
                .unwrap()
        }
    };

    word

}


#[launch]
fn rocket() -> _ {  
    //Create new shuffle file in the background  
    thread::spawn(|| { make_shuffle_file(); });

    rocket::build()
        .mount("/", routes![get_current_word])
}