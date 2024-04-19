use crate::bit_hash::{sha1, sha256, sha512};

pub struct BloomFilter {
    bits: u32,
}

impl BloomFilter {
    pub fn new() -> Self {
        BloomFilter { bits: 0 }
    }

    pub fn add(&mut self, item: &str) {
        for hash in Self::hashes(item) {
            self.set(hash);
        }
    }

    pub fn contains(&self, item: &str) -> bool {
        for hash in Self::hashes(item) {
            if !self.is_set(hash) {
                return false;
            };
        }
        true
    }

    fn hashes(item: &str) -> Vec<u32> {
        vec![sha1(item), sha256(item), sha512(item)]
    }

    fn bit_index(hash: u32) -> u32 {
        hash % 32
    }

    fn set(&mut self, hash: u32) {
        let index = Self::bit_index(hash);
        self.bits |= 1 << index;
    }

    fn is_set(&self, hash: u32) -> bool {
        let index = Self::bit_index(hash);
        (self.bits & (1 << index)) != 0
    }
}
