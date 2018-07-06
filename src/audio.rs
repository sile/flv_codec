use bytecodec::{ErrorKind, Result};

/// AAC packet type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AacPacketType {
    /// AAC sequence header
    SequenceHeader = 0,

    /// AAC raw
    Raw = 1,
}
impl AacPacketType {
    pub(crate) fn from_u8(b: u8) -> Result<Self> {
        Ok(match b {
            0 => AacPacketType::SequenceHeader,
            1 => AacPacketType::Raw,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown aac packet type: {}", b),
        })
    }
}

/// Audio format(codec) identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundFormat {
    /// Linear PCM,platform endian
    LinearPcmPlatformEndian = 0,

    /// ADPCM
    Adpcm = 1,

    /// MP3
    Mp3 = 2,

    /// Linear PCM, little endian
    LinearPcmLittleEndian = 3,

    /// Nellymoser 16-kHz mono
    Nellymoser16khzMono = 4,

    /// Nellymoser 8-kHz mono
    Nellymoser8KhzMono = 5,

    /// Nellymoser
    Nellymoser = 6,

    /// G.711 A-law logarithmic PCM
    G711AlawLogarithmicPcm = 7,

    /// G.711 mu-law logarithmic PCM
    G711MuLawLogarithmicPcm = 8,

    /// AAC
    Aac = 10,

    /// Speex
    Speex = 11,

    /// MP3 8-kHz
    Mp3_8khz = 14,

    ///  Device-specific sound
    DeviceSpecificSound = 15,
}
impl SoundFormat {
    pub(crate) fn from_u8(b: u8) -> Result<Self> {
        Ok(match b {
            0 => SoundFormat::LinearPcmPlatformEndian,
            1 => SoundFormat::Adpcm,
            2 => SoundFormat::Mp3,
            3 => SoundFormat::LinearPcmLittleEndian,
            4 => SoundFormat::Nellymoser16khzMono,
            5 => SoundFormat::Nellymoser8KhzMono,
            6 => SoundFormat::Nellymoser,
            7 => SoundFormat::G711AlawLogarithmicPcm,
            8 => SoundFormat::G711MuLawLogarithmicPcm,
            10 => SoundFormat::Aac,
            11 => SoundFormat::Speex,
            14 => SoundFormat::Mp3_8khz,
            15 => SoundFormat::DeviceSpecificSound,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown FLV sound format: {}", b),
        })
    }
}

/// Audio sampling rate.
///
/// Note that if the format is `SoundFormat::Aac`, `SoundRate::Khz44` always be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SoundRate {
    /// 5.5-kHz
    Khz5 = 0,

    /// 11-kHz
    Khz11 = 1,

    /// 22-kHz
    Khz22 = 2,

    /// 44-kHz
    Khz44 = 3,
}
impl SoundRate {
    pub(crate) fn from_u8(b: u8) -> Result<Self> {
        Ok(match b {
            0 => SoundRate::Khz5,
            1 => SoundRate::Khz11,
            2 => SoundRate::Khz22,
            3 => SoundRate::Khz44,
            _ => track_panic!(ErrorKind::InvalidInput, "Unknown FLV sound rate: {}", b),
        })
    }
}

/// Size of each audio sample.
///
/// Note that this parameter only pertains to uncompressed formats.
/// Compressed formats always decode to 16 bits internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SoundSize {
    /// 8-bits
    Bit8 = 0,

    /// 16-bits
    Bit16 = 1,
}
impl SoundSize {
    pub(crate) fn from_bool(b: bool) -> Self {
        if b {
            SoundSize::Bit16
        } else {
            SoundSize::Bit8
        }
    }
}

/// Mono or stereo sound.
///
/// Nellymoser and AAC always use `Mono` and `Stereo` respectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SoundType {
    /// Monoral
    Mono = 0,

    /// Stereo
    Stereo = 1,
}
impl SoundType {
    pub(crate) fn from_bool(b: bool) -> Self {
        if b {
            SoundType::Stereo
        } else {
            SoundType::Mono
        }
    }
}
