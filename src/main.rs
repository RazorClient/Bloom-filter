// src/main.rs

use log::error;
use std::path::Path;

use bloom::{BloomFilter, read_string_input, read_usize_input, select_operation};

fn main() {
    // Initialize the logger
    env_logger::init();

    println!("Welcome to the Bloom Filter CLI!");

    // Prompt user for number of hash functions
    let num_hash_functions = loop {
        let num = read_usize_input("Enter the number of hash functions to use (3 or 4): ");
        if num >= 3 && num <= 4 {
            break num;
        } else {
            println!("Number of hash functions must be 3 or 4.");
        }
    };

    // Prompt user for array size
    let array_size = read_usize_input("Enter the size of the bit array (positive integer): ");

    // Prompt user for number of levels
    let num_levels = read_usize_input("Enter the number of levels (positive integer): ");

    // Create the BloomFilter
    let mut bloom_filter = match BloomFilter::new(num_levels, array_size, num_hash_functions) {
        Ok(bf) => {
            println!("Bloom Filter created successfully!");
            bf
        },
        Err(e) => {
            error!("Error creating BloomFilter: {}", e);
            println!("Error creating BloomFilter: {}", e);
            return;
        }
    };

    // Allow operations on the bloom filter
    loop {
        let selection = select_operation();

        match selection {
            0 => { // Insert item
                let item = read_string_input("Enter item to insert: ");
                bloom_filter.insert(&item);
                println!("Item inserted successfully.");
            },
            1 => { // Query item
                let item = read_string_input("Enter item to query: ");
                let levels_to_search = loop {
                    let levels = read_usize_input("Enter number of levels to search: ");
                    if levels > 0 && levels <= num_levels {
                        break levels;
                    } else {
                        println!("Number of levels to search must be between 1 and {}.", num_levels);
                    }
                };
                let found = bloom_filter.query(&item, levels_to_search);
                if found {
                    println!("Item may be present.");
                } else {
                    println!("Item is not present.");
                }
            },
            2 => { // Save Bloom Filter
                let filepath = read_string_input("Enter the filepath to save the Bloom Filter (e.g., bloom.json): ");
                if let Err(e) = bloom_filter.save_to_file(&filepath) {
                    error!("Failed to save BloomFilter: {}", e);
                    println!("Failed to save BloomFilter: {}", e);
                } else {
                    println!("Bloom Filter saved successfully.");
                }
            },
            3 => { // Load Bloom Filter
                let filepath = read_string_input("Enter the filepath to load the Bloom Filter from (e.g., bloom.json): ");
                if !Path::new(&filepath).exists() {
                    println!("File does not exist. Please enter a valid filepath.");
                    continue;
                }
                match BloomFilter::load_from_file(&filepath) {
                    Ok(bf) => {
                        bloom_filter = bf;
                        println!("Bloom Filter loaded successfully.");
                    },
                    Err(e) => {
                        error!("Failed to load BloomFilter: {}", e);
                        println!("Failed to load BloomFilter: {}", e);
                    }
                }
            },
            4 => { // Exit
                println!("Exiting the Bloom Filter CLI. Goodbye!");
                break;
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
