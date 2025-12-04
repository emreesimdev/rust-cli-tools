use rand::Rng;

fn main() {
    println!("RANDOM PIN GENERATOR 🔐");
    println!("Generating a random 6- digit pin");
    println!("--------------------------------");
    print!("Your secure PIN: ");

    // Let's generate a 6-digit password
    for _ in 0..6 {
        let num = rand::thread_rng().gen_range(0..=9);
        print!("{}", num);
    }
    
    println!(""); // To move on to the next line
    println!("--------------------------------");
}