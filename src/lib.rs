#[macro_use]
extern crate bytecodec;
#[macro_use]
extern crate trackable;

pub use file::FileDecoder;
pub use header::Header;
pub use tag::{
    AacPacketType, AudioTag, AvcPacketType, CodecId, CompositionTimeOffset, FrameType,
    ScriptDataTag, SoundFormat, SoundRate, SoundSize, SoundType, StreamId, Tag, TagDecoder,
    TagType, Timestamp, VideoTag,
};

mod file;
mod header;
mod tag;
