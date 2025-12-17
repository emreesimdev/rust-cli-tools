use rand::Rng;

use std::io;

fn main(){
    println!("---Hangman Game Started---");

    // Static word list (Rust-themed)
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

    // Pick a random word from the list
    let random_index = rand::thread_rng().gen_range(0..words.len());
    let secret_word = words[random_index];

    // Vector that holds revealed characters
    let mut revealed: Vec<char> = Vec::new();

    // Initialize hidden word with underscores
    for _ in 0..secret_word.len() {
        revealed.push('_');
    }

    // Number of wrong attempts allowed
    let mut attempts_left = 6;

    loop {
        // Show current state of the word
        println!("\nWord: {:?}", revealed,);
        
        // Get user input
        let guess = get_input("Guess a letter:");

        // Only allow single character input
        if guess.len() != 1 {
            println!("Please enter a single letter.");
            continue;
        }
        
        let guess_char = guess.chars().next().unwrap();

        let mut found = false;

        // Check guessed letter against the secret word
        for (i, c) in secret_word.chars().enumerate() {
            if c == guess_char {
                revealed [i] = c;
                found = true;
            }
        }

        // Win condition
        if !revealed.contains(&'_') {
            println!("You won! The word was: {}", secret_word);
            break;
        }

        // Wrong guess handling
        if !found {
            attempts_left -= 1;
            println!("Wrong guess! Attemps left: {}",attempts_left);
        }

        // Lose condition
        if attempts_left == 0 {
            println!("Game over! The word was: {}", secret_word);
            break;
        }
        
    };

}

// Simple reusable input function
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}