/// A generic hash that can be referenced as slice of bytes.
pub trait Hash: AsRef<[u8]> {}

/// A hasher that produces a `Hash` of OUTPUT_LENGTH bytes from a given message.
pub trait Hasher {
    /// NOTE: This is in bytes, not bits.
    const BLOCK_LENGTH: usize;

    /// NOTE: This is in bytes, not bits.
    const OUTPUT_LENGTH: usize;

    /// NOTE: A `Hasher` can return any custom type for the hash, but it must implement AsRef<[u8]> so users can get the hash as a raw slice of bytes.
    type Output: Hash;

    /// Update the input state of the hasher with the provided message bytes.
    fn update(&mut self, msg: &[u8]);

    /// Compute the final hash with the hasher state.
    fn finalize(self) -> Self::Output;

    /// Stateless hashing of a given message.
    fn hash(msg: &[u8]) -> Self::Output;
}
