mod bit_hash;
mod bloom_filter;

use bit_hash::{sha1, sha256, sha512};
use bloom_filter::BloomFilter;

fn main() {
    // hash functions
    let input = "foo";
    let sha1 = sha1(input);
    let sha256 = sha256(input);
    let sha512 = sha512(input);

    println!("hashes for {:?}", input);
    println!("sha1 = {}", sha1);
    println!("sha256 = {}", sha256);
    println!("sha512 = {}", sha512);

    // simple bloom filter
    let mut bloom = BloomFilter::new();

    bloom.add("foo");

    if bloom.contains("foo") {
        println!("contains foo");
    } else {
        println!("no foo");
    }

    if bloom.contains("bar") {
        println!("contains bar");
    } else {
        println!("no bar");
    }

    bloom.add("bar");

    if bloom.contains("bar") {
        println!("contains bar");
    } else {
        println!("no bar");
    }
}
