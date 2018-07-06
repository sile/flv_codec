use bytecodec::{ErrorKind, Result};

/// Video codec identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodecId {
    /// JPEG (currently unused)
    Jpeg = 1,

    /// Sorenson H.263
    H263 = 2,

    /// Screen Video
    ScreenVideo = 3,

    /// On2 VP6
    Vp6 = 4,

    /// On2 VP6 with alpha channel
    Vp6WithAlpha = 5,

    /// Screen video version 2
    ScreenVideoV2 = 6,

    /// AVC
    Avc = 7,
}
impl CodecId {
    pub(crate) fn from_u8(b: u8) -> Result<Self> {
        Ok(match b {
            1 => CodecId::Jpeg,
            2 => CodecId::H263,
            3 => CodecId::ScreenVideo,
            4 => CodecId::Vp6,
            5 => CodecId::Vp6WithAlpha,
            6 => CodecId::ScreenVideoV2,
            7 => CodecId::Avc,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown video codec ID: {}", b),
        })
    }
}

/// Video frame type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameType {
    /// Key frame (for AVC, a seekable frame)
    KeyFrame = 1,

    /// Inter frame (for AVC, a non-seekable frame)
    InterFrame = 2,

    /// Disposable inter frame (H.263 only)
    DisposableInterFrame = 3,

    /// Generated key frame (reserved for server use only)
    GeneratedKeyFrame = 4,

    /// Video info/command frame
    VideoInfoOrCommandFrame = 5,
}
impl FrameType {
    pub(crate) fn from_u8(b: u8) -> Result<Self> {
        Ok(match b {
            1 => FrameType::KeyFrame,
            2 => FrameType::InterFrame,
            3 => FrameType::DisposableInterFrame,
            4 => FrameType::GeneratedKeyFrame,
            5 => FrameType::VideoInfoOrCommandFrame,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown video frame type: {}", b),
        })
    }
}

/// AVC packet type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvcPacketType {
    /// AVC sequence header
    SequenceHeader = 0,

    /// AVC NALU
    NalUnit = 1,

    /// AVC end of sequence
    ///
    /// Lower level NALU sequence ender is not required or supported.
    EndOfSequence = 2,
}
impl AvcPacketType {
    pub(crate) fn from_u8(b: u8) -> Result<Self> {
        Ok(match b {
            0 => AvcPacketType::SequenceHeader,
            1 => AvcPacketType::NalUnit,
            2 => AvcPacketType::EndOfSequence,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown AVC packet type: {}", b),
        })
    }
}
