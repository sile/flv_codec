use bytecodec::{ErrorKind, Result};

/// Stream identifier.
///
/// Ordinally, the identifier always be set to `0` (the default value).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StreamId(u32);
impl StreamId {
    /// Makes a new `StreamId` instance.
    ///
    /// # Errors
    ///
    /// If `id` is greater than `0xFF_FFFF`, it will return an `ErrorKind::InvalidInput` error.
    pub fn new(id: u32) -> Result<Self> {
        track_assert!(id <= 0xFF_FFFF, ErrorKind::InvalidInput; id);
        Ok(StreamId(id))
    }

    /// Returns the value of the identifier.
    pub fn value(self) -> u32 {
        self.0
    }
}
impl Default for StreamId {
    fn default() -> Self {
        StreamId(0)
    }
}
