use std::{collections::HashSet, fs::read_to_string};

fn process_to_nato(repo: &[&str], mut input: String) {
    input = input[0..64.min(input.len())].to_string().to_uppercase();
    for ch in input.chars() {
        if ch.is_alphabetic() {
            let idx = (ch as usize) - ('A' as usize); // b'A' is u8
            print!("{} ", repo[idx]);
        }
    }
}

fn input_conv(repo: &[&str]) {
    let mut input = String::new();

    println!("Enter conv candidate");
    std::io::stdin()
        .read_line(&mut input) // BufReader would be better but we gotta have a life
        .expect("Failed to read line");

    process_to_nato(repo, input);
}

fn process_from_nato(repo: &[&str], input: &str) {
    let repo_set = repo.iter().map(|w| *w).collect::<HashSet<&str>>();
    let mut words = input.split(" ");
    for word in words.by_ref() {
        if repo_set.contains(word) {
            print!("{}", &word[0..1]);
        }
    }
}

fn main() {
    let nato = [
        "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India",
        "Juliett", "Kilo", "Lima", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo",
        "Sierra", "Tango", "Uniform", "Victor", "Whiskey", "Xray", "Yankee", "Zulu",
    ];
    process_from_nato(&nato, "Alfa Bravo Charlie Delta");
    println!("\n----------------------------------------------------");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        input_conv(&nato);
    } else {
        // work with files
        // open it or create one by this name
        let contents = read_to_string(&args[1]);
        if let Err(e) = contents {
            println!("error : {}", e);
            return;
        }
        process_to_nato(&nato, contents.unwrap());
    }
}
