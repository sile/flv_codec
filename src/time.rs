use bytecodec::{ErrorKind, Result};
use std;
use std::time::Duration;

/// 32-bits signed timestamp in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Timestamp(i32);
impl Timestamp {
    /// Makes a new `Timestamp` instance.
    pub fn new(milliseconds: i32) -> Self {
        Timestamp(milliseconds)
    }

    /// Returns the value of this timestamp in milliseconds.
    pub fn value(self) -> i32 {
        self.0
    }

    /// Converts `Timestamp` to `Duration`.
    ///
    /// If the value of this timestamp is negative, it will return `None`.
    pub fn to_duration(self) -> Option<Duration> {
        if self.0 >= 0 {
            Some(Duration::from_millis(self.0 as u64))
        } else {
            None
        }
    }

    /// Converts `Duration` to `Timestamp`.
    ///
    /// # Errors
    ///
    /// If the value of `duration` is too large (i.e., greater than `i32::MAX`),
    /// it will return an `ErrorKind::InvalidInput` error.
    pub fn from_duration(duration: Duration) -> Result<Self> {
        let milliseconds = duration.as_secs() * 1000 + u64::from(duration.subsec_millis());
        track_assert!(
            milliseconds <= std::i32::MAX as u64,
            ErrorKind::InvalidInput;
            duration
        );
        Ok(Timestamp(milliseconds as i32))
    }
}

/// 24-bits signed timestamp offset in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeOffset(i32);
impl TimeOffset {
    /// Makes a new `TimeOffset` instance.
    ///
    /// # Errors
    ///
    /// If `offset` is out-of-range of signed 24-bit integers,
    /// it will return an `ErrorKind::InvalidInput` error.
    pub fn new(offset: i32) -> Result<Self> {
        track_assert_eq!(((offset << 8) >> 8), offset, ErrorKind::InvalidInput);
        Ok(TimeOffset(offset))
    }

    /// Returns the value of this time offset.
    pub fn value(self) -> i32 {
        self.0
    }

    pub(crate) fn from_u24(n: u32) -> Self {
        TimeOffset(((n << 8) as i32) >> 8)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn timestamp_works() {
        // signed
        let t = Timestamp::new(123);
        assert_eq!(t.value(), 123);
        assert_eq!(t.to_duration(), Some(Duration::from_millis(123)));
        assert_eq!(
            Some(t),
            Timestamp::from_duration(Duration::from_millis(123)).ok()
        );

        // unsigned
        let t = Timestamp::new(-123);
        assert_eq!(t.value(), -123);
        assert_eq!(t.to_duration(), None);

        // too large
        assert_eq!(
            None,
            Timestamp::from_duration(Duration::from_secs(0xFFFF_FFFF)).ok()
        );
    }

    #[test]
    fn time_offset_works() {
        assert_eq!(TimeOffset::new(123).map(|t| t.value()).ok(), Some(123));
        assert_eq!(TimeOffset::new(-12).map(|t| t.value()).ok(), Some(-12));
        assert!(TimeOffset::new(0x0080_0000).is_err());
        assert!(TimeOffset::new(0x0080_0000 - 1).is_ok());
        assert!(TimeOffset::new(-0x0080_0000).is_ok());
        assert!(TimeOffset::new(-0x0080_0000 - 1).is_err());
    }
}
