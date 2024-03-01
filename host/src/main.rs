// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use json_core::Outputs;
use json_methods::SEARCH_JSON_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    let data = include_str!("../../res/sample.json");
    let outputs = search_json(data);
    println!();
    println!("  {:?}", outputs.hash);
    println!(
        "provably contains a field 'critical_data' meets the threshold value is {}",
        outputs.result
    );
}

fn search_json(data: &str) -> Outputs {
    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, SEARCH_JSON_ELF).unwrap();

    receipt.journal.decode().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_critical_data() {
        let data = include_str!("../../res/sample.json");
        let outputs = super::search_json(data);
        assert_eq!(
            outputs.result, true,
            "Critical data field meets the expected threshold value"
        );
    }
}
