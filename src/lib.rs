/// A hash represented as a fixed array of bytes.
pub type Hash<const SIZE: usize> = [u8; SIZE];

/// A hash function that produces a `Hash` of OUTPUT_SIZE bytes from a given message.
pub trait HashFn<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize>: Sized {
    /// Initialize hash state.
    fn new() -> Self;

    /// Update the hash state with the given data.
    fn update(&mut self, data: &[u8]);

    /// Consumes the hash state, producing the final hash value.
    fn finalize(self) -> Hash<OUTPUT_SIZE>;

    /// Produces the final hash value, resetting the hash state for reuse.
    fn finalize_reset(&mut self) -> Hash<OUTPUT_SIZE>;

    /// Produces a hash for the given data.
    fn hash(data: &[u8]) -> Hash<OUTPUT_SIZE> {
        let mut hasher = Self::new();

        hasher.update(data);

        hasher.finalize()
    }
}
