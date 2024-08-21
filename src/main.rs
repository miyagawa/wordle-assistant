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

fn mask_answer(ans: &mut Vec<char>, c: char) {
    for i in 0..ans.len() {
        if ans[i] == c {
            ans[i] = '_';
            break;
        }
    }
}

#[derive(Debug)]
struct Guess {
    word: String,
    status: String,
}

impl Guess {
    fn build_status(&self, answer: &str) -> String {
        let mut ans : Vec<char> = answer.chars().collect();
        self.word.chars().enumerate().map(|(i, c)| {
            if ans[i] == c {
                mask_answer(&mut ans, c);
                'G'
            } else if ans.contains(&c) {
                mask_answer(&mut ans, c);
                'Y'
            } else {
                'B'
            }
        }).collect()
    }

    fn satisfy(&self, answer: &str) -> bool {
        self.build_status(answer) == self.status
    }
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
            .filter(|&word| guesses.iter().all(|guess| guess.satisfy(word)))
            .for_each(|word| println!("{}", word));
    }
}

