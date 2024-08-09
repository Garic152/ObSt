use chrono::prelude::*;
use rusqlite::{Connection, Result};
use std::io;
use std::path::Path;

static DATATYPES: [&str; 2] = ["INTEGER", "FLOAT"];

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

fn create_table(observation: Observation) -> Result<Connection> {
    println!("Setting up observation.");

    let path = Path::new("./observations/observations.db");
    let conn = Connection::open(&path)?;

    let mut columns = String::new();
    for param in &observation.parameters {
        columns.push_str(&format!("{} {}, ", param[0], param[1]));
    }
    columns.pop();
    columns.pop();

    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} ({});",
        observation.name, columns
    );
    println!("{}", sql);
    conn.execute(&sql, [])?;

    Ok(conn)
}

fn new_observation() {
    let name = prompt_user("What do you want to observe? (String)");
    if name.is_empty() {
        println!("Obersvation name cannot be empty. Exiting...");
        return;
    }

    let mut parameters: Vec<Vec<String>> = Vec::new();

    if prompt_user("Want to add a date for each entry? [Y/n]") == "Y" {
        parameters.push(vec![String::from("Date"), String::from("DATETIME")]);
        println!("Added a date element to the table");
    } else {
        println!("Input not [Y]es, skipping date...");
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

    for i in 0..param_num {
        let arg_name = prompt_user(&format!("Name of argument {}", i + 1));

        let arg_type = prompt_user(&format!("Type of argument {}:\n[1] Integer\n[2] Float", i));

        match arg_type.trim().parse::<usize>() {
            Ok(number) => {
                if number > DATATYPES.len() {
                    println!("Invalid data type, exiting...\n");
                    return;
                }
                parameters.push(vec![arg_name, String::from(DATATYPES[number - 1])]);
            }
            Err(_) => {
                println!("Number not valid, exiting...\n");
                return;
            }
        };
        println!("\n");
    }
    let observation = Observation { name, parameters };

    let result = create_table(observation);
    match result {
        Ok(_) => {
            println!("Successfully created observation, exiting...");
            return;
        }
        Err(e) => {
            println!("Failed to create observation: {}", e);
            return;
        }
    }
}

fn main() {
    println!("Welcome to ObSt beta!\nSelect one of the following options by typing the corresponding number.");
    println!("1: Start new observation");
    println!("2: Add to an observation");

    let input = prompt_user("Enter your choice:");

    match input.parse::<u8>() {
        Ok(1) => new_observation(),
        Ok(_) => println!("Invalid Option"),
        Err(_) => println!("Please enter a valid number."),
    }
}
