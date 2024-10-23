// src/utils.rs

use dialoguer::{Input, Select};
use log::error;

/// Reads a positive integer from the user with a prompt.
pub fn read_usize_input(prompt: &str) -> usize {
    loop {
        match Input::<String>::new()
            .with_prompt(prompt)
            .interact_text()
        {
            Ok(input) => {
                if let Ok(num) = input.trim().parse::<usize>() {
                    if num > 0 {
                        return num;
                    } else {
                        println!("Please enter a positive integer.");
                    }
                } else {
                    println!("Invalid input. Please enter a valid number.");
                }
            }
            Err(e) => {
                error!("Error reading input: {}", e);
                println!("An error occurred while reading input. Please try again.");
            }
        }
    }
}

/// Reads a non-empty string from the user with a prompt.
pub fn read_string_input(prompt: &str) -> String {
    loop {
        match Input::<String>::new()
            .with_prompt(prompt)
            .interact_text()
        {
            Ok(input) => {
                let trimmed = input.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                } else {
                    println!("Input cannot be empty. Please enter a valid string.");
                }
            }
            Err(e) => {
                error!("Error reading input: {}", e);
                println!("An error occurred while reading input. Please try again.");
            }
        }
    }
}

/// Presents a selection menu to the user and returns the selected index.
pub fn select_operation() -> usize {
    let operations = vec![
        "Insert item",
        "Query item",
        "Save Bloom Filter",
        "Load Bloom Filter",
        "Exit",
    ];
    let selection = Select::new()
        .with_prompt("Choose an operation")
        .items(&operations)
        .default(0)
        .interact_opt()
        .unwrap_or(None);

    match selection {
        Some(index) => index,
        None => 4, // Default to "Exit" if no selection is made
    }
}
