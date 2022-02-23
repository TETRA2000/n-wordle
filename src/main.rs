use std::fs;
use std::io;
use rand::seq::SliceRandom;

fn main() {
    println!("Loading dictionary...");

    // Read dictionary file
    let contents = fs::read_to_string("/usr/share/dict/words").unwrap();
    // Split dictionary file into words
    let words: Vec<&str> = contents.split("\n").collect();
    println!("Loaded {} words", words.len());

    println!("How many characters of word do you want to guess? [3-6]");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

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
        io::stdin().read_line(&mut buffer);
        let guess = buffer.trim();

        let mut found = false;
        if answer == guess {
            println!("You guessed correctly!");
            found = true;
            break;
        }
        if !found {
            println!("You guessed incorrectly!");
        }
    }

    println!("The answer was {}", answer);
}
