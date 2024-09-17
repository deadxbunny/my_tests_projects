mod random_game;
mod training_to_rust_lang;

use random_game::{game, game_2};
use std::{io, process::exit};

fn app_menu(){
    println!("[1] rand_game\n[2] none\n[3] none\n[4] none\n[5] exit program");
    println!("Select number");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    match buffer.trim().parse::<u32>() {
        Err(e) => { println!("Error: {}", e); },
        Ok(1) => { game() },
        Ok(2) => { println!("{}", game_2()) },
        Ok(3) => { println!("3") },
        Ok(4) => { println!("4") },
        Ok(5) => { println!("exit program, goodbye!"); exit(0); },
        _ => {println!("Wrong input")}
    }
}

fn main() {
    loop {
        app_menu();
        println!("Push enter to continue...");
        io::stdin().read_line( & mut String::from("\n") ).unwrap();
    }
}