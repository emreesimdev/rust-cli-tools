use rand::Rng;

use std::{char, io};

fn main(){
    println!("---Hangman Game Started---");

    let words = vec![
        "borrow",
        "ownership",
        "compiler",
        "lifetime",
        "memory",
        "vector",
        "struct",
        "enum",
    ];

    let random_index = rand::thread_rng().gen_range(0..words.len());
    let secret_word = words[random_index];

    let mut revealed: Vec<char> = Vec::new();

    for _ in 0..secret_word.len() {
        revealed.push('_');
    }

    loop {
        println!("\nWord: {:?}", revealed,);
        
        let guess = get_input("Guess a letter:");

        if guess.len() != 1 {
            println!("Please enter a single letter.");
            continue;
        }
        
        let guess_char = guess.chars().next().unwrap();

        let mut found = false;

        for (i, c) in secret_word.chars().enumerate() {
            if c == guess_char {
                revealed [i] = c;
                found = true;
            }
        }

        if !found {
            println!("Wrong guess!");
        }
    };

}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}