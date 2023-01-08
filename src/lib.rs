/// A hash represented by a fixed array of bytes.
pub type Hash<const SIZE: usize> = [u8; SIZE];

/// A hash function that produces a `Hash` of OUTPUT_SIZE bytes from a given message.
pub trait HashFn<const BLOCK_SIZE: usize, const OUTPUT_SIZE: usize> {
    /// Compute the has for the given message.
    fn hash(msg: impl IntoIterator) -> Hash<OUTPUT_SIZE>;
}
