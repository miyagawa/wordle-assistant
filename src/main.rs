use std::error;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn read_words_file() -> Result<Vec<String>> {
    let file = File::open("/usr/share/dict/words")?;

    Ok(io::BufReader::new(file).lines()
       .flatten()
       .filter_map(|word| {
           match (word.len(), word.chars().nth(0).unwrap()) {
               (5, 'a'..='z') => Some(word),
               _ => None,
           }
       })
       .collect())
}

#[derive(Debug)]
struct Guess {
    word: String,
    status: String,
}

fn satisfy(answer: &str, guess: &Guess) -> bool {
    let status: String = guess.word.chars().enumerate().map(|(i, c)| {
        match (answer.contains(c), answer.chars().nth(i)) {
            (true, Some(a)) if a == c => 'G',
            (true, Some(a)) if a != c => 'Y',
            _ => 'B',
        }
    }).collect();

    status == guess.status
}

fn main() {
    let words = read_words_file()
        .expect("Could not read file");

    let re = Regex::new(r"^([a-zA-Z]{5}) ([YGB]{5})$").unwrap();
    let mut guesses = Vec::new();

    loop {
        println!("---> Enter your guess, followed by YGB (Yellow, Green, Black)");
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line);
        if let Ok(0) = n {
            std::process::exit(0);
        }
        if let Some(caps) = re.captures(&line.trim_end()) {
            let word = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let status = caps.get(2).map_or("", |m| m.as_str()).to_string();

            guesses.push(Guess { word, status });
        } else {
            eprintln!("Invalid input.");
            continue;
        }

        words.iter()
            .filter(|&word| guesses.iter().all(|guess| satisfy(word, guess)))
            .for_each(|word| println!("{}", word));
    }
}

