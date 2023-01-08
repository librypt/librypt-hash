/// A generic hash that can be referenced as slice of bytes.
pub type Hash<const SIZE: usize> = [u8; SIZE];

/// A hasher that produces a `Hash` of OUTPUT_SIZE bytes from a given message.
pub trait Hasher<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize> {
    /// Initialize incremental hasher.
    fn new() -> Self;

    /// Update the input state of the hasher with the provided message bytes.
    fn update(&mut self, msg: &[u8]);

    /// Compute the final hash with the hasher state.
    fn finalize(self) -> Hash<OUTPUT_SIZE>;

    /// Compute the final hash with the hasher state, clearing the state for reuse.
    fn finalize_reset(&mut self) -> Hash<OUTPUT_SIZE>;

    /// Stateless hashing of a given message.
    fn hash(msg: &[u8]) -> Hash<OUTPUT_SIZE>;
}
