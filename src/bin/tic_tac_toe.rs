use std::io;

fn main() {

    // 1. SETUP: Create a 3x3 board filled with dashes
    let mut board = vec![
        vec!['-', '-', '-'],
        vec!['-', '-', '-'],
        vec!['-', '-', '-'],
    ];

    // Starting player
    let mut current_player = 'X';

    // 2. GAME LOOP
    loop {
        
        println!("---Current Board---");

        for row in &board {
            println!("{:?}", row);
        }
        
        println!("Current Player: {}",current_player);
        println!("(Type 'q' to quit)");

        // INPUT LOGIC

        // Row Input
        let row_input = get_input("Enter a row (0-2)");

        // QUIT CHECK for row
        if row_input == "q" {
            println!("Goodbye!");
            break;
        }

        let row_no: usize = match row_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid number!");
                continue;
            }
        };

        // Column Input
        let column_input = get_input("Enter a column (0-2)");

        // QUIT CHECK for column
        if column_input == "q" {
            println!("Goodbye!");
            break;
        }

        let column_no: usize = match column_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // SAFETY CHECKS

        if row_no > 2 || column_no > 2 {
            println!("Error: You must enter 0, 1, or 2! You've gone off the board.");
            continue;
        }

        if board[row_no][column_no] != '-'  {
            println!("That place is already full! Choose somewhere else.");
            continue;
        }

        // UPDATE BOARD
        board[row_no][column_no] = current_player;

        // SWITCH TURN
        if current_player == 'X' {
            current_player = 'O';
        } else {
            current_player = 'X';
        }

    }
}

// HELPER FUNCTION
fn get_input(promt: &str) -> String {
    println!("{}", promt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}