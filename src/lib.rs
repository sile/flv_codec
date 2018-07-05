#[macro_use]
extern crate bytecodec;
#[macro_use]
extern crate trackable;

pub use body::FlvBodyDecoder;
pub use header::{FlvHeader, FlvHeaderDecoder};
pub use tag::{
    AudioTag, FlvTag, FlvTagDecoder, ScriptDataTag, StreamId, TagType, Timestamp, VideoTag,
};

mod body;
mod header;
mod tag;
