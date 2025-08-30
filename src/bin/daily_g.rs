use chrono::{Datelike, Local, Timelike};
use rand::Rng;
use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) // yes I am not buffering
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn fortune() -> String {
    let lines = read_lines("fortune.txt");
    let mut rng = rand::rng();
    let fortune_idx = rng.random_range(0..lines.len());
    lines[fortune_idx].to_string()
}

fn moon_phase(year: i32, month: u32, day: u32) -> u32 {
    let mut d = day;
    if month == 2 {
        d = d + 31;
    } else if month > 2 {
        d = d + 59 + ((((month - 3) as f32 * 30.6) as u32) as f32 + 0.5) as u32;
    }
    let g = (year - 1900) % 19;
    let mut e = (11 * g + 29) % 30;
    if e == 25 || e == 24 {
        e = e + 1;
    }
    ((((e + d as i32) * 6 + 5) % 177) / 22 & 7) as u32
}

fn main() {
    let idx_to_mon_name: HashMap<u32, &str> = HashMap::from([
        (1, "Jan"),
        (2, "Feb"),
        (3, "Mar"),
        (4, "Apr"),
        (5, "May"),
        (6, "Jun"),
        (7, "Jul"),
        (8, "Aug"),
        (9, "Sep"),
        (10, "Oct"),
        (11, "Nov"),
        (12, "Dec"),
    ]);

    let phase = vec![
        "waxing crescent",
        "at first quarter",
        "waxing gibbous",
        "full",
        "waning gibbous",
        "at last quarter",
        "waning crescent",
        "new",
    ];

    let time = Local::now();
    let hour = time.hour();
    print!("Good ");
    if hour < 12 {
        print!("morning");
    } else if hour < 17 {
        print!("afternoon");
    } else {
        print!("evening");
    }

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() >= 2 {
        print!(" {}", args[1]);
    }
    println!();
    println!(
        "Today is {}, {} {}, {}",
        time.weekday(),
        idx_to_mon_name.get(&(time.month())).unwrap(),
        time.day(),
        time.year()
    );
    println!("It is {}", time.time());
    let mp = moon_phase(time.year(), time.month(), time.day());
    println!("The moon is {}", phase[mp as usize]);
    println!("{}", fortune());
}
