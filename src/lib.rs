/// The raw bytes for a hash.
pub type Hash<const LENGTH: usize> = [u8; LENGTH];

/// A hasher that produces a `Hash` of HASH_LENGTH from a given message.
pub trait Hasher<const HASH_LENGTH: usize> {
    fn update(&mut self, msg: &[u8]);

    fn finalize(self) -> Hash<HASH_LENGTH>;

    fn hash(msg: &[u8]) -> Hash<HASH_LENGTH>;
}
