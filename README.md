# SP1 Project Template

This is a template for creating an end-to-end [SP1](https://github.com/succinctlabs/sp1) project
that can generate a proof of any RISC-V program.

## Purpose

This project helps evaluate:

- The computational efficiency of using SP1’s patched precompiles (`tiny-keccak`).

## Requirements

- [Rust](https://rustup.rs/)
- [SP1 SDK](https://docs.succinct.xyz/getting-started/install.html)

## Running the Project

### Build the Program

```sh
cd program
cargo prove build
```

### Execute the Program

To run the program **without generating a proof**, use one of the following commands:

#### Run with **Patched SP1 tiny-keccak**:

```sh
RUST_LOG=info cargo run --release -- --execute --precompile
```

#### Run with **Non-patched (tiny-keccak)**:

```sh
RUST_LOG=info cargo run --release -- --execute --without-precompile
```

The output will display:

- The input message.
- The selected hash implementation (`precompile` or `without-precompile`).
- The computed Keccak256 hash.
- The total cycle count reported by the SP1 VM.

### Generate a Core Proof

To generate a proof for your program without using precompile:

```sh
cd script
cargo run --release -- --prove --without-precompile
```

To generate a proof for your program using precompile:

```sh
cd script
cargo run --release -- --prove --precompile
```

Observe `prover-core` cycles summary!
