/// A hash that can be referenced as a fixed array of bytes.
pub trait Hash<const SIZE: usize>: AsRef<[u8; SIZE]> {}

/// A hash function that produces a `Hash` of OUTPUT_SIZE bytes from a given message.
pub trait HashFn<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize>: Sized {
    /// Custom output hash type (must implement `Hash` trait).
    type Output: Hash<OUTPUT_SIZE>;

    /// Initialize hash state.
    fn new() -> Self;

    /// Update the hash state with the given data.
    fn update(&mut self, data: &[u8]);

    /// Consumes the hash state, producing the final hash value.
    fn finalize(self) -> Self::Output;

    /// Produces the final hash value, resetting the hash state for reuse.
    fn finalize_reset(&mut self) -> Self::Output;

    /// Compute the has for the given message.
    fn hash(msg: &[u8]) -> Self::Output {
        let mut hasher = Self::new();

        hasher.update(msg);

        hasher.finalize()
    }
}
