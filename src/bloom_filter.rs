// src/bloom_filter.rs

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use log::{info, error};
use thiserror::Error;

/// Custom error type for BloomFilter operations.
#[derive(Error, Debug)]
pub enum BloomFilterError {
    #[error("Serialization/Deserialization Error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),

    #[error("Invalid number of hash functions. Requested: {requested}, Available: {available}")]
    InvalidHashFunctions { requested: usize, available: usize },
}

/// Represents a Bloom Filter with multiple levels.
#[derive(Serialize, Deserialize)]
pub struct BloomFilter {
    levels: Vec<BloomLevel>,
    hash_functions: Vec<HashFunction>,
    array_size: usize,
}

impl BloomFilter {
    /// Creates a new BloomFilter with the specified number of levels, array size, and hash functions.
    pub fn new(num_levels: usize, array_size: usize, num_hash_functions: usize) -> Result<Self, BloomFilterError> {
        info!(
            "Creating BloomFilter: levels={}, array_size={}, hash_functions={}",
            num_levels, array_size, num_hash_functions
        );
        // Create the hash functions
        let multipliers = vec![31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        if num_hash_functions > multipliers.len() {
            error!(
                "Requested hash functions ({}) exceed available ({})",
                num_hash_functions,
                multipliers.len()
            );
            return Err(BloomFilterError::InvalidHashFunctions {
                requested: num_hash_functions,
                available: multipliers.len(),
            });
        }
        let hash_functions: Vec<HashFunction> = multipliers[..num_hash_functions]
            .iter()
            .map(|&multiplier| HashFunction::new(multiplier))
            .collect();

        // Create levels
        let levels = (0..num_levels)
            .map(|_| BloomLevel::new(array_size))
            .collect();

        Ok(BloomFilter {
            levels,
            hash_functions,
            array_size,
        })
    }

    /// Inserts an item into all levels of the Bloom filter.
    pub fn insert(&mut self, item: &str) {
        info!("Inserting item: {}", item);
        for level in &mut self.levels {
            level.insert(item, &self.hash_functions, self.array_size);
        }
    }

    /// Queries an item across the specified number of levels.
    pub fn query(&self, item: &str, num_levels_to_search: usize) -> bool {
        info!("Querying item: {} across {} levels", item, num_levels_to_search);
        let levels_to_search = std::cmp::min(num_levels_to_search, self.levels.len());
        for i in 0..levels_to_search {
            if self.levels[i].query(item, &self.hash_functions, self.array_size) {
                return true;
            }
        }
        false
    }

    /// Saves the Bloom filter to a file in JSON format.
    pub fn save_to_file(&self, filepath: &str) -> Result<(), BloomFilterError> {
        info!("Saving BloomFilter to file: {}", filepath);
        let file = File::create(filepath)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }

    /// Loads a Bloom filter from a JSON file.
    pub fn load_from_file(filepath: &str) -> Result<Self, BloomFilterError> {
        info!("Loading BloomFilter from file: {}", filepath);
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let bloom_filter = serde_json::from_reader(reader)?;
        Ok(bloom_filter)
    }
}

/// Represents a single level within the Bloom filter.
#[derive(Serialize, Deserialize)]
pub struct BloomLevel {
    bit_array: Vec<bool>,
}

impl BloomLevel {
    /// Creates a new BloomLevel with the specified array size.
    pub fn new(array_size: usize) -> Self {
        BloomLevel {
            bit_array: vec![false; array_size],
        }
    }

    /// Inserts an item into the BloomLevel using the provided hash functions.
    pub fn insert(&mut self, item: &str, hash_functions: &[HashFunction], array_size: usize) {
        for hf in hash_functions {
            let hash = hf.hash(item) % array_size;
            self.bit_array[hash] = true;
        }
    }

    /// Queries an item in the BloomLevel using the provided hash functions.
    pub fn query(&self, item: &str, hash_functions: &[HashFunction], array_size: usize) -> bool {
        for hf in hash_functions {
            let hash = hf.hash(item) % array_size;
            if !self.bit_array[hash] {
                return false;
            }
        }
        true
    }
}

/// Represents a single hash function used in the Bloom filter.
#[derive(Serialize, Deserialize)]
pub struct HashFunction {
    multiplier: usize,
}

impl HashFunction {
    /// Creates a new HashFunction with the specified multiplier.
    pub fn new(multiplier: usize) -> Self {
        HashFunction { multiplier }
    }

    /// Computes the hash of a string.
    pub fn hash(&self, s: &str) -> usize {
        s.bytes()
            .fold(0, |hash, b| hash.wrapping_mul(self.multiplier).wrapping_add(b as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_query() {
        let mut bf = BloomFilter::new(1, 100, 3).unwrap();
        bf.insert("test");
        assert!(bf.query("test", 1));
        assert!(!bf.query("nonexistent", 1));
    }

    #[test]
    fn test_save_and_load() {
        let mut bf = BloomFilter::new(1, 100, 3).unwrap();
        bf.insert("test");
        bf.save_to_file("test_bloom.json").unwrap();

        let loaded_bf = BloomFilter::load_from_file("test_bloom.json").unwrap();
        assert!(loaded_bf.query("test", 1));
        assert!(!loaded_bf.query("nonexistent", 1));

        // Clean up test file
        std::fs::remove_file("test_bloom.json").unwrap();
    }
}
