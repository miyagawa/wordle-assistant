use std::error;
use std::fs::File;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn read_words_file() -> Result<Vec<String>> {
    let file = File::open("/usr/share/dict/words")?;

    let words = io::BufReader::new(file).lines()
        .flatten()
        .filter_map(|word| {
            match (word.len(), word.chars().nth(0).unwrap()) {
                (5, 'a'..='z') => Some(word),
                _ => None,
            }
        })
        .collect();

    Ok(words)
}

#[derive(Debug)]
struct Guess {
    word: String,
    status: String,
}

fn satisfy(answer: &str, guess: &Guess) -> bool {
    let mut status = String::new();
    for (i, c) in guess.word.chars().enumerate() {
        let s = if answer.chars().nth(i) == Some(c) {
            'G'
        } else if answer.contains(c) && answer.chars().nth(i) != Some(c) {
            'Y'
        } else {
            'B'
        };
        status.push(s);
    }

    status == guess.status
}

fn main() {
    let words = read_words_file()
        .expect("Could not read file");

    let mut guesses = vec![];

    loop {
        println!("---> Enter your guess, followed by YGB (Yellow, Green, Black)");
        let mut line = String::new();
        match io::stdin().read_line(& mut line) {
            Ok(0) => {
                std::process::exit(0);
            },
            Ok(_) => {
                let guess : Vec<&str> = line.split_whitespace().collect();
                match guess.len() {
                    2 => {
                        guesses.push(Guess { word: guess[0].to_string(), status: guess[1].to_string() });
                    },
                    _ => {
                        eprintln!("Invalid input.");
                        continue;
                    },
                };
            },
            _ => {},
        };

        words.iter()
            .filter(|&word| guesses.iter().all(|guess| satisfy(word, guess)))
            .for_each(|word| println!("{}", word));
    }
}

