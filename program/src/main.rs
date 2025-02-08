#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::FixedBytes;
use alloy_sol_types::SolValue;
use hasher_lib::{hash_keccak_with_precompile, hash_keccak_without_precompile, PublicHashResult};
use sp1_zkvm::io;

pub fn main() {
    let flag: u8 = io::read();
    let message = io::read::<Vec<u8>>();

    // Showcase purposes only lol
    let use_patched = flag != 0;
    let hash = if use_patched {
        hash_keccak_with_precompile(&message)
    } else {
        hash_keccak_without_precompile(&message)
    };

    let public = PublicHashResult {
        input: message.into(),
        patched: use_patched,
        hash: FixedBytes(hash),
    };

    let bytes = public.abi_encode();
    io::commit_slice(&bytes);
}
