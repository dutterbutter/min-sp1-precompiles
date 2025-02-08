//! You can run this script using the following command:
//! ```shell
//! # Execute the program using the patched version.
//! RUST_LOG=info cargo run --release -- --execute --patched
//! ```
//! or
//! ```shell
//! # Execute the program using the un-patched version.
//! RUST_LOG=info cargo run --release -- --execute --un-patched
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
use hasher_lib::PublicHashResult;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

pub const HASH_PROGRAM_ELF: &[u8] = include_elf!("hasher-program");

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "Hello, SP1!")]
    message: String,

    /// Use patched version of tiny-keccak.
    #[clap(long)]
    patched: bool,

    /// Use un-patched version of tiny-keccak.
    #[clap(long = "un-patched")]
    un_patched: bool,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse command-line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }
    if args.patched == args.un_patched {
        eprintln!("Error: You must specify either --patched or --un-patched");
        std::process::exit(1);
    }
    let use_patched: bool = args.patched;

    // Initialize the prover client.
    let client = ProverClient::from_env();

    // Set up the input to the program.
    let mut stdin = SP1Stdin::new();

    // First write a flag indicating which implementation to use.
    let flag: u8 = if use_patched { 1 } else { 0 };
    stdin.write(&flag);

    stdin.write(&args.message.as_bytes());

    if args.execute {
        let (output, report) = client.execute(HASH_PROGRAM_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        let decoded = PublicHashResult::abi_decode(output.as_slice(), true)
            .expect("Failed to decode public values");
        let PublicHashResult {
            input,
            patched,
            hash,
        } = decoded;

        println!("Input (hex): {}", hex::encode(input));
        println!(
            "Implementation used: {}",
            if patched { "patched" } else { "un-patched" }
        );
        println!("Computed hash: {}", hex::encode(hash));

        println!(
            "Total cycles reported: {}",
            report.total_instruction_count()
        );
        println!("Number of constraints: {}", report.total_syscall_count());
    } else {
        // Set up the proving keys.
        let (pk, vk) = client.setup(HASH_PROGRAM_ELF);

        // Generate the proof.
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Proof generated successfully!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Proof verified successfully!");
    }
}
