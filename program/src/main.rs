#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::FixedBytes;
use alloy_sol_types::SolValue;
use hasher_lib::{hash_keccak_precompile, hash_keccak_sw, PublicHashResult};
use sp1_zkvm::io;

pub fn main() {
    let flag: u8 = io::read();
    let message = io::read::<Vec<u8>>();

    // Showcase purposes only lol
    let use_patched = flag != 0;
    let hash = if use_patched {
        hash_keccak_precompile(&message)
    } else {
        hash_keccak_sw(&message)
    };

    // Package the public output.
    let public = PublicHashResult {
        input: message.into(),
        patched: use_patched,
        hash: FixedBytes(hash),
    };

    let bytes = public.abi_encode();
    io::commit_slice(&bytes);
}
