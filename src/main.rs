use rusqlite::{Connection, Result};
use std::fs::File;
use std::io;
use std::path::Path;

static DATATYPES: [&str; 3] = ["INTEGER", "BOOLEAN", "float"];

struct Observation {
    name: String,
    parameters: Vec<Vec<String>>,
}

fn prompt_user(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    input.trim().to_string()
}

fn create_table(observation: Observation) {
    let path = Path::new("../obervations/observations.db");
    let display = path.display();

    match File::create(&path) {
        Err(why) => println!(
            "DB file {} already exists or couldn't be created: {}",
            display, why
        ),
        Ok(_) => println!("DB file created!"),
    };
}

fn new_observation() {
    let name = prompt_user("What do you want to observe? (String)");
    if name.is_empty() {
        println!("Obersvation name cannot be empty. Exiting...");
        return;
    }

    let param_num =
        prompt_user("How many correlated parameters do you want to track? (Number value)");

    let param_num = match param_num.trim().parse::<u8>() {
        Ok(number) => {
            println!("You chose {} parameters, which you can define now.", number);
            number
        }
        Err(_) => {
            println!("No valid number (<256), exiting...\n");
            return;
        }
    };

    let mut parameters: Vec<Vec<String>> = Vec::new();

    for i in 0..param_num {
        let arg_name = prompt_user(&format!("Name of argument {}", i + 1));

        let arg_type = prompt_user(&format!(
            "Type of argument {}:\n[1] Integer\n[2] Bool\n[3] Float",
            i
        ));

        match arg_type.trim().parse::<usize>() {
            Ok(number) => {
                if number > DATATYPES.len() - 1 {
                    println!("Invalid data type, exiting...\n");
                    return;
                }
                parameters.push(vec![arg_name, String::from(DATATYPES[number])]);
            }
            Err(_) => {
                println!("Number not valid, exiting...\n");
                return;
            }
        };
        println!("\n");
    }
    let observation = Observation { name, parameters };
    create_table(observation);
}

fn main() {
    println!("Welcome to ObSt beta!\nSelect one of the following options by typing the corresponding number.");
    println!("1: Start new observation\n");

    let input = prompt_user("Enter your choice:");

    match input.parse::<u8>() {
        Ok(1) => new_observation(),
        Ok(_) => println!("Invalid Option"),
        Err(_) => println!("Please enter a valid number."),
    }
}
