use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(0..=1);

    if random_number == 0 {
        println!("COIN FLIP: HEADS");
    } else {
        println!("COIN FLIP: TAILS");
    }
}
