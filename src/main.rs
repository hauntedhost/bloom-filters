use digest::Digest;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

/// Compute a 32-bit numeric hash of input str based on given hashing algorithm
fn hash<D: Digest>(input: &str) -> u32 {
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

fn sha1(input: &str) -> u32 {
    hash::<Sha1>(input)
}

fn sha256(input: &str) -> u32 {
    hash::<Sha256>(input)
}

fn sha512(input: &str) -> u32 {
    hash::<Sha512>(input)
}

fn main() {
    let input = "foo";

    let sha1 = sha1(input);
    let sha256 = sha256(input);
    let sha512 = sha512(input);

    println!("sha1 = {}", sha1);
    println!("sha256 = {}", sha256);
    println!("sha512 = {}", sha512);
}
