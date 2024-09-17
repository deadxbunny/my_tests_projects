use std::io;
use rand::Rng;

pub fn game() {
    println!("Введи число от 0 до 5");

    // read user line
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    // convert string to int
    let buffer_int:i32 = buffer.trim().parse().expect("Please type a number!");

    // generate random int
    let rand_int = rand::thread_rng().gen_range(0..=5);

    // check user int to rand generate
    if buffer_int == rand_int {
        println!("Success!");
    }
        else if buffer_int > 5 {
            println!("WARNING!");
        }
    else {
        println!("Ops...try again..");
    }

    println!("Random int: {}", rand_int);
}

pub fn game_2 () ->&'static str {
    "prosto text"
}

