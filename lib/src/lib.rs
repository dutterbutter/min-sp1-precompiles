use alloy_sol_types::sol;

sol! {
    /// The public values for the hash example.
    /// Contains the input message, a flag indicating which implementation was used,
    /// and the computed hash.
    struct PublicHashResult {
        bytes input;
        bool patched;
        bytes32 hash;
    }
}

pub fn hash_keccak_precompile(input: &[u8]) -> [u8; 32] {
    println!("Using patched precompile for keccak256 hash computation.");
    use patched_tiny_keccak::{Hasher, Keccak};
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut output);
    output
}

pub fn hash_keccak_sw(input: &[u8]) -> [u8; 32] {
    println!("Using non-precompile for keccak256 hash computation.");
    use unpatched_tiny_keccak::{Hasher as UnPatchedHasher, Keccak as UnPatchedKeccak};
    let mut keccak = UnPatchedKeccak::v256();
    let mut output = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut output);
    output
}
