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
enum Status {
    YELLOW,
    GREEN,
    BLACK,
}

#[derive(Debug)]
struct Character {
    char: char,
    status: Status,
    pos: usize,
}

#[derive(Debug)]
struct Guess {
    chars: Vec<Character>,
}

fn build_guess(word: &str, status: &str) -> Guess {
    let pair = word.chars().zip(status.chars());
    let mut chars = vec![];
    for (i, (char, st)) in pair.enumerate() {
        let status = match st {
            'Y' => Status::YELLOW,
            'G' => Status::GREEN,
            'B' => Status::BLACK,
            _ => Status::BLACK,
        };
        let pos = i;
        chars.push(Character { char, status, pos })
    }

    Guess { chars }
}

fn satisfy(word: &str, guess: &Guess) -> bool {
    for char in &guess.chars {
        let ok = match char.status {
            Status::YELLOW => {
                word.contains(char.char) && word.chars().nth(char.pos) != Some(char.char)
            },
            Status::GREEN => {
                word.chars().nth(char.pos) == Some(char.char)
            },
            Status::BLACK => {
                !word.contains(char.char)
            },
        };

        if !ok {
            return false;
        }
    }

    true
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
                guesses.push(build_guess(guess[0], guess[1]));
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

