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
    precompile: bool,

    /// Use un-patched version of tiny-keccak.
    #[clap(long = "without-precompile")]
    without_precompile: bool,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }
    if args.precompile == args.without_precompile {
        eprintln!("Error: You must specify either --precompile or --without-precompile");
        std::process::exit(1);
    }
    let use_patched: bool = args.precompile;

    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();

    let flag: u8 = if use_patched { 1 } else { 0 };
    stdin.write(&flag);
    stdin.write(&args.message.as_bytes());

    if args.execute {
        let (output, report) = client.execute(HASH_PROGRAM_ELF, &stdin).run().unwrap();

        let decoded = PublicHashResult::abi_decode(output.as_slice(), true)
            .expect("Failed to decode public values");
        let PublicHashResult {
            input,
            patched,
            hash,
        } = decoded;

        println!(
            "Execution: {}",
            if patched {
                "Using Keccak256 precompile"
            } else {
                "Without Keccak256 precompile"
            }
        );
        println!("Input (hex): {}", hex::encode(input));
        println!("Computed hash: {}", hex::encode(hash));
        println!(
            "Total cycles reported: {}",
            report.total_instruction_count()
        );
    } else {
        let (pk, vk) = client.setup(HASH_PROGRAM_ELF);

        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Proof generated successfully!");

        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Proof verified successfully!");
    }
}
