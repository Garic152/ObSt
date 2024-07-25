use rusqlite::{Connection, Result};
use std::io;

fn new_observation() {
    println!("Creating new observation.");
}

fn main() {
    println!("Welcome to obst beta!\nSelect one of the following options by typing the corresponding number.");
    println!("1: Start new observation");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let trimmed_input = input.trim();
    match trimmed_input.parse::<u8>() {
        Ok(number) => {
            if number == 1 {
                new_observation();
            }
        }
        Err(_) => {
            println!("Please enter a valid number.");
        }
    }
}
