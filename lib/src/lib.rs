use alloy_sol_types::sol;

sol! {
    /// The public values for the hash example.
    /// Contains the input message, a flag indicating if precompile was used,
    /// and the computed hash.
    struct PublicHashResult {
        bytes input;
        bool patched;
        bytes32 hash;
    }
}

pub fn hash_keccak_with_precompile(input: &[u8]) -> [u8; 32] {
    println!("Using precompile for keccak256 hash computation.");
    use patched_tiny_keccak::{Hasher, Keccak};
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut output);
    output
}

pub fn hash_keccak_without_precompile(input: &[u8]) -> [u8; 32] {
    println!("Using non-precompile for keccak256 hash computation.");
    use unpatched_tiny_keccak::{Hasher as UnPatchedHasher, Keccak as UnPatchedKeccak};
    let mut keccak = UnPatchedKeccak::v256();
    let mut output = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut output);
    output
}
