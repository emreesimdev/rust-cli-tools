use std::io;
use rand::Rng;

fn main() {
    println!("🔫 RUSSIAN ROULETTE 🔫");

    // Put the bullet in the chamber
    let bullet_slot = rand::thread_rng().gen_range(1..=6);

    // Turn the cylinder and stop it somewhere
    let mut current_chamber = rand::thread_rng().gen_range(1..=6);

    let mut bounty = 0;
    let mut round = 1;

    loop {
        println!("\n--- Round {} | Current Bounty: {}$ ---",round, bounty);
        println!("1. Pull THE TRIGGER");
        println!("2. CASH OUT (Take money and leave)");

        println!("What is your choice?");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice_int: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice_int {
            1 =>  {
                // Pull The Trigger
                println!("Click...");
    
                if current_chamber == bullet_slot {
                    println!("\n💥 BOOM! 💥");
                    println!("The hammer hit the bullet in chamber {}.", current_chamber);
                    println!("YOU DIED. Game Over. You lost EVERYTHING.");
                    break;
                } else {
                    // Survived
                    println!("\n...Empty.");
                    println!("Cylinder rotates to the next chamber.");

                    current_chamber += 1;

                    if current_chamber > 6 {
                        current_chamber = 1;
                    }
                    // Reward: The greater the risk, the greater the reward.
                    bounty += 100 * round;
                    round += 1;
                }
            }
            2 => {
                // Cash Out
                println!("\n💰 Smart move! You walked away with {}$", bounty);
                break;
            }
            _ => {
                // Invalid Choice
                println!("Invalid choice! Please type '1' or '2'.");
                continue;
            }
        }
    }
}