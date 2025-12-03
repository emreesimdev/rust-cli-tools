use rand::Rng;

fn main() {
    println!("⚔️ ROLLING FOR INITIATIVE...(d20) ⚔️");

    // Generate numbers between 1 and 20
    let roll = rand::thread_rng().gen_range(1..=20);

    println!("---------------------------------------");

    match roll {
        1 => println!("🎲 Result: 1 (CRITICAL MISS!)"), // Worst luck
        20 => println!("🎲 Result: 20 (CRITICAL HIT!)"), // Best luck
        _ => println!("🎲 Result: {}", roll), // Normal result
    }

    println!("---------------------------------------");
}