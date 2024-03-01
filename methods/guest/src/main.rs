#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use json::parse;
use json_core::Outputs;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);


fn main() {
    // Read input JSON from the environment
    let data: String = env::read();

    // Calculate SHA hash of the input data
    let sha = *Impl::hash_bytes(&data.as_bytes());

    // Parse the input data fields
    let data = parse(&data).unwrap();

    // Isolate the 'critical_data' field from the parsed JSON
    let critical_data_check = data["critical_data"].as_u32();

    // Check 'critical_data' field isn't empty, if so, extract it
    let critical_data = match critical_data_check {
        Some(value) => value,
        None => {
            // Handle the case where 'critical_data' is empty or not present
            eprintln!("The 'critical_data' field is empty or not present.");
            // Returning a default value of 0
            0
        }
    };

    // Define the minimum threshold for critical data
    let threshold_val = 50;

    // Assume the critical data is not proven until evaluated
    let mut proven = false;

    // Evaluate whether critical data meets the minimum threshold
    if critical_data > threshold_val {
        proven = true; // Set proven to true if the condition is met
    }

    // Define the output struct containing critical data, hash, and evaluation result
    let out = Outputs {
        data: critical_data,
        hash: sha,
        result: proven,
    };

    // Commit the output struct to the environment
    env::commit(&out);
}
