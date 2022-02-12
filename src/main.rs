use std::fs::File;
use std::io::{self, BufRead};

fn read_words_file() -> Result<Vec<String>, std::io::Error> {
    let file = File::open("/usr/share/dict/words")?;

    let lines = io::BufReader::new(file).lines();
    let mut words = vec![];
    for line in lines {
        if let Ok(word) = line {
            let first = word.chars().nth(0).unwrap();
            if word.len() == 5 && first >= 'a' && first <= 'z'  {
                words.push(word);
            }
        }
    }

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
        if let Ok(_) = io::stdin().read_line(& mut line) {
            let guess : Vec<&str> = line.split_whitespace().collect();
            if guess.len() == 2 {
                guesses.push(Guess { word: guess[0].to_string(), status: guess[1].to_string() });
            } else {
                eprintln!("Invalid input.");
            }
        } else {
            eprintln!("Invalid input.");
        }

        'words: for word in &words {
            for guess in &guesses {
                if !satisfy(word, guess) {
                    continue 'words;
                }
            }
            println!("{}", word);
        }
    }
}

