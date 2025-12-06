use std::io;

fn main() {
    println!("🏥 BMI CALCULATOR (Body Mass Index) 🏥");

    println!("Enter your weight (kg):");
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).expect("Error");

    let weight: f64 = match weight_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a valid number!");
            return;
        }
    };

    println!("Enter your height: (meters, e.g., 1.80)");
    let mut height_input = String::new();
    io::stdin().read_line(&mut height_input).expect("Error");

    let height: f64 = match height_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a valid number!");
            return;
        }
    };

    let bmi = weight / (height * height);
    println!("------------------------------------");

    if bmi < 18.5 {
        println!("Status: Underweight 🦴");
    }  else if bmi < 25.0 {
        println!("Status: Normal Weight ✅");
    } else if bmi < 30.0 {
        println!("Status: Overweight ⚠️");
    } else {
        println!("Status: Obese 🚨");
    }
    println!("------------------------------------");
}