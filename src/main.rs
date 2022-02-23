use std::fs;
use std::io;
use ansi_term::Color::{Black, Green, White, Yellow};
use rand::seq::SliceRandom;
use ansi_term::Style;


fn main() {
    println!("Loading dictionary...");

    // Read dictionary file
    let contents = fs::read_to_string("/usr/share/dict/words").unwrap();
    // Split dictionary file into words
    let words: Vec<&str> = contents.split("\n").collect();
    println!("Loaded {} words", words.len());

    println!("How many characters of word do you want to guess? [3-6]");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    // TODO: validate input

    // Convert string to int
    let num_chars = buffer.trim().parse::<u8>().unwrap();
    println!("You chose {} characters", num_chars);

    let mut sub_words: Vec<&str> = Vec::new();
    for word in words {
        if word.len() == num_chars as usize {
            // println!("{}", word);
            sub_words.push(word);
        }
    }
    println!("{} words in dictionary", sub_words.len());

    // Select random from sub_words
    let answer = *(sub_words.choose(&mut rand::thread_rng()).unwrap());

    // Do 5 times
    for _ in 0..5 {
        println!("Guess a word");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let guess = buffer.trim();

        if answer == guess {
            print!("{}", Style::new().on(Green).fg(Black).paint(answer));
            println!("You guessed correctly!");
            break;
        }

        // Split word into chars
        let guess_chars: Vec<char> = guess.chars().collect();
        let answer_chars: Vec<char> = answer.chars().collect();
        for i in 0..answer_chars.len() {
            if guess_chars[i] == answer_chars[i] {
                print!("{}", Style::new().on(Green).fg(Black).paint(guess_chars[i].to_string()));
            } else if answer_chars.contains(&guess_chars[i]) {
                print!("{}", Style::new().on(Yellow).fg(Black).paint(guess_chars[i].to_string()));
            } else {
                print!("{}", Style::new().on(Black).fg(White).paint(guess_chars[i].to_string()));
            }
        }
        print!("\n");
        println!("You guessed incorrectly!");
    }

    println!("The answer was {}", answer);
}
