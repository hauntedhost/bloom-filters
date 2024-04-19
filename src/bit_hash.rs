use digest::Digest;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

// Compute a 32-bit numeric hash of input str based on given hashing algorithm
fn bit_hash<D: Digest>(input: &str) -> u32 {
    let mut hasher = D::new();
    hasher.update(input);

    // Hash result as a byte array
    let result = hasher.finalize();

    // Convert the first 8 bytes to a big-endian u64 from the slice
    let big_endian_u64 = u64::from_be_bytes(
        result[..8]
            .try_into()
            .expect("Expected at least 8 bytes from hash output"),
    );

    // Reduce the u64 to u32 by capturing the lower 32 bits
    (big_endian_u64 % 0x100000000) as u32
}

pub fn sha1(input: &str) -> u32 {
    bit_hash::<Sha1>(input)
}

pub fn sha256(input: &str) -> u32 {
    bit_hash::<Sha256>(input)
}

pub fn sha512(input: &str) -> u32 {
    bit_hash::<Sha512>(input)
}
